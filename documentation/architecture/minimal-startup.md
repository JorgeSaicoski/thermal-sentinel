# Minimal Startup — Layers and Contracts

This document uses one concrete example — reading CPU data and wrapping it in a domain type — to explain what each layer is for, what a contract is, and why contracts live in the domain.

Read [project-structure.md](project-structure.md) first if you haven't. This document goes deeper on the *why* and then shows the complete, working implementation.

> **Note on approach:** This document shows the full code for every file — an intentional exception to the usual learning pattern. Normally you figure out the implementation yourself from concept explanations alone. Here, seeing the complete architecture once first helps you build a mental model of how the layers connect. The suggested use: read it all the way through, then close it and build each file from memory. The compiler will tell you what you got wrong.

---

## What is a contract?

A contract is a type that two separate parts of the program agree on. One side produces it, the other consumes it — and neither side needs to know anything about how the other works.

In Rust, a `struct` is a contract. When you define:

```rust
pub struct CpuInfo {
    pub temperature: f32,
    pub usage: f32,
}
```

You are saying: *anyone who reads CPU hardware must give back this shape, and anyone who uses CPU data must expect this shape*. The two sides never meet — they only know about the struct.

---

## Why contracts live in the domain

The domain layer is the innermost layer. It has no dependencies on anything external — no crates, no hardware, no network. This makes it the only safe place to define types that the whole program shares.

If `CpuInfo` lived in `infra/sensors.rs` instead, the `app` layer would have to import from `infra` just to name the type. That breaks the dependency rule — app would be reaching into infra for something that has nothing to do with hardware.

By putting `CpuInfo` in `domain`, every layer can import it without crossing a boundary in the wrong direction:

```
domain  ←  infra   (sensors produces CpuInfo)
domain  ←  app     (snapshot consumes CpuInfo)
```

Both arrows point inward. Neither layer knows about the other.

---

## The example: CPU data through the layers

Here is how `CpuInfo` travels from hardware to output, one layer at a time. Each section below shows the complete file to create — no pseudocode, no gaps to fill in.

---

### Domain — define the contract

The domain layer is two files: one for `CpuInfo`, one for `Reading`. They contain only struct definitions. No `use` statements, no external crates.

**`src/domain/cpu_info.rs`**

```rust
pub struct CpuInfo {   // pub struct — visible to other modules; without pub, only domain/ could use it
    pub temperature: f32, // pub field — other modules can read this value; f32 is a 32-bit floating point number
    pub usage: f32,
}
```

**`src/domain/reading.rs`**

`Reading` holds a `CpuInfo` as a field rather than storing `temperature` and `usage` as separate flat values. This grouping reflects that the two values have the same origin — they come from the same adapter at the same moment.

```rust
use crate::domain::cpu_info::CpuInfo; // crate:: means "start from the root of this project" — it's how Rust navigates your own modules

pub struct Reading {
    pub timestamp: String,
    pub cpu: CpuInfo,
    pub outdoor_temp: Option<f32>,  // Option means this value may or may not exist — None if unknown, Some(42.0) if present
    pub city: Option<String>,       // same pattern — city is optional until a weather adapter fills it in
}
```

**`src/domain/mod.rs`**

Rust does not automatically discover files in a folder. You must declare each submodule explicitly:

```rust
pub mod cpu_info;
pub mod reading;
```

`pub mod` makes the submodule visible to code outside `domain`. Without `pub`, nothing else can use it.

---

### Infra — produce the contract

`sensors` is the only part of the program that knows how to read CPU hardware. It imports `CpuInfo` from the domain and returns one. Everything about `sysinfo` — the `Components` type, the double-refresh pattern, the `Option<f32>` temperature — is hidden inside this layer.

**`src/infra/sensors.rs`**

```rust
use std::thread;
use sysinfo::{Component, Components, System};
use crate::domain::cpu_info::CpuInfo;

pub fn read() -> CpuInfo {
    let components = Components::new_with_refreshed_list(); // ask sysinfo for all hardware sensors on the machine

    let temperature = components
        .iter()                                    // walk through each sensor one at a time
        .find_map(|c: &Component| c.temperature()) // for each sensor, try to get its temperature — stop at the first one that has one
        .unwrap_or(0.0);                           // .temperature() returns Option<f32>; if every sensor returned None, fall back to 0.0

    let mut sys = System::new();
    sys.refresh_cpu_usage();                               // first snapshot — on its own this number is meaningless
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);   // wait the minimum time sysinfo requires between reads
    sys.refresh_cpu_usage();                               // second snapshot — sysinfo computes usage as the delta between the two
    let usage = sys.global_cpu_usage(); // now the percentage is valid — this is a single average across all cores

    CpuInfo { temperature, usage } // shorthand: when a variable name matches a field name, Rust lets you write it once
}
```

The caller receives a `CpuInfo`. It never sees `sysinfo`. If you later replace `sysinfo` with direct kernel reads (see [sysinfo.md](../crates/sysinfo.md)), this is the only file that changes.

> **One value by design:** `global_cpu_usage()` returns a single average across all cores — not per-core data. This is intentional for the minimal: one number is enough to see the full architecture work. When you're ready to go further, `sys.cpus()` returns a list of individual cores you can iterate over.

Note the import direction: `infra` imports from `domain`. Never the other way.

**`src/infra/mod.rs`**

```rust
pub mod sensors;
```

---

### Why sensors doesn't return a `Reading`

`sensors` only knows two of the four fields on `Reading`. If it returned a `Reading`, it would have to fill `outdoor_temp: None` and `city: None` — making a claim about weather from inside a hardware adapter. That is not its responsibility.

When you add a weather adapter later, you would have two separate adapters each trying to build an incomplete `Reading`, stepping on each other's fields.

The rule: **each adapter returns what it knows**. The app layer assembles the whole.

---

### App — assemble the whole

`snapshot` is the composition root. It is the only place that calls all adapters and builds a complete `Reading`. It knows about both `CpuInfo` (via the sensors call) and `Reading` (from the domain), but it knows nothing about `sysinfo` or HTTP.

**`src/app/snapshot.rs`**

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::reading::Reading;
use crate::infra::sensors;

pub fn take() -> Reading {
    let cpu = sensors::read();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH) // seconds elapsed since Jan 1 1970 — the standard Unix timestamp
        .unwrap_or_default()        // duration_since returns Result, not Option; .unwrap_or_default() means "use zero if this somehow fails"
        .as_secs()                  // convert the Duration value into a plain u64 (whole seconds)
        .to_string();               // turn the number into a String so Reading.timestamp can hold it

    Reading {
        timestamp,
        cpu,
        outdoor_temp: None, // no weather adapter yet — None signals "this field has no value"
        city: None,
    }
}
```

`cpu` is placed into `Reading` as a unit. No field-by-field unpacking. This is one of the benefits of grouping related values into a named type — the assembly is clean.

**`src/app/mod.rs`**

```rust
pub mod snapshot;
```

---

### Interface — consume without knowing origins

The display layer receives a `Reading` and formats it. It does not know whether temperature came from `sysinfo` or direct kernel reads. It just formats what it receives.

**`src/interface/display.rs`**

```rust
use crate::domain::reading::Reading;

pub fn show(reading: &Reading) { // & means we borrow Reading — we can read its fields but we don't take ownership of it
    println!(
        "[{}] CPU: {:.1} °C | Usage: {:.1}%",
        //           ^^^^ format specifier: print this f32 with exactly 1 decimal place
        reading.timestamp, reading.cpu.temperature, reading.cpu.usage
    );
}
```

**`src/interface/mod.rs`**

```rust
pub mod display;
```

---

### Entry point — wire everything together

`main.rs` declares all top-level modules and calls the two functions that drive the program. Nothing else happens here.

**`src/main.rs`**

```rust
mod app;      // mod (no pub) — declares the module for use inside main.rs only; contrast with pub mod inside mod.rs files
mod domain;
mod infra;
mod interface;

fn main() {
    let reading = app::snapshot::take();
    interface::display::show(&reading); // & passes a reference — show reads but doesn't consume reading
}
```

---

## The full flow

```
hardware
  ↓
infra::sensors::read()        →  produces  CpuInfo
                                               ↓
                    app::snapshot::take()   →  produces  Reading
                                                            ↓
                                   interface::display::show()
```

At every boundary, the only thing that crosses is a domain type. No layer leaks its internals to the next.

---

## Running it

Once all files are in place:

```bash
cargo run
```

Expected output:

```
[1720123456] CPU: 42.0 °C | Usage: 8.3%
```

Your numbers will be different — they come from your machine's hardware at the moment you run it. Here is what each part means:

```
[1720123456]   CPU: 42.0 °C   |   Usage: 8.3%
 ───────────        ────             ────
     │                │                └─ percentage of CPU capacity in use right now
     │                └─ temperature of the first sensor that reported a value, in Celsius
     └─ Unix timestamp — seconds elapsed since January 1, 1970
        (paste it into https://www.unixtimestamp.com to see the human-readable date)
```

A few things worth knowing about these values:

- **42.0 °C** is a normal idle temperature for a CPU. Under load, it might climb to 70–90 °C. Above 95 °C the hardware starts throttling itself. This is what we are eventually monitoring.
- **8.3%** means the CPU spent about 8% of its time doing work during the measurement window. The double-refresh in `sensors.rs` is what makes this number meaningful — one snapshot alone cannot compute a rate.
- **The timestamp** is a raw integer here. In a later step you will switch to a human-readable format using the `chrono` crate.

---

## Applying the pattern to new features

Every new data source follows the same four steps. No existing file changes except the ones that directly own the new concern.

**Step 1 — Domain**: what struct holds this data? Add it in `domain/` and declare it in `domain/mod.rs`.

**Step 2 — Infra**: what external system provides it? Write an adapter in `infra/` that imports the domain struct and returns it. Add it to `infra/mod.rs`.

**Step 3 — App**: where does it get assembled? In `app/snapshot.rs` — call the new adapter and fill in its fields on `Reading`.

**Step 4 — Interface**: how does the user see it? In `interface/display.rs` — read the new field from `Reading` and format it.

### Example: outdoor temperature via HTTP

- `domain/` — `Reading` already has `outdoor_temp: Option<f32>` and `city: Option<String>`
- `infra/weather.rs` — fetches from an API, returns a struct with city and temperature; add `pub mod weather;` to `infra/mod.rs`
- `app/snapshot.rs` — call `weather::fetch()`, fill in the two fields
- `interface/display.rs` — add outdoor temp to the output line

### Example: a terminal UI with a crate like `ratatui`

The pattern is the same. `ratatui` belongs in `interface/` — it is a display concern. The data it renders comes from a `Reading` passed in as a reference. The rest of the architecture does not change.

- `interface/tui.rs` — draws the terminal UI using a `&Reading`; add `pub mod tui;` to `interface/mod.rs`
- `main.rs` — call `interface::tui::show(&reading)` instead of (or alongside) `display::show`

The architectural rule: `interface/` is the only layer that knows about visual output. `app/` and `infra/` stay unaware of how data is presented.

---

## Further reading

- [project-structure.md](project-structure.md) — the full target structure across all five modes
- [sysinfo.md](../crates/sysinfo.md) — details on the double-refresh pattern and direct kernel reads
- [rust_memory.md](../rust/rust_memory.md) — why `show` takes `&Reading` instead of `Reading`
