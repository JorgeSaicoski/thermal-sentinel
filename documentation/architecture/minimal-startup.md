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
pub struct CpuInfo {
    pub temperature: f32,
    pub usage: f32,
}
```

**`src/domain/reading.rs`**

`Reading` holds a `CpuInfo` as a field rather than storing `temperature` and `usage` as separate flat values. This grouping reflects that the two values have the same origin — they come from the same adapter at the same moment.

```rust
use crate::domain::cpu_info::CpuInfo;

pub struct Reading {
    pub timestamp: String,
    pub cpu: CpuInfo,
    pub outdoor_temp: Option<f32>,
    pub city: Option<String>,
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
    let components = Components::new_with_refreshed_list();
    let temperature = components
        .iter()
        .find_map(|c: &Component| c.temperature())
        .unwrap_or(0.0);

    let mut sys = System::new();
    sys.refresh_cpu_usage();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    let usage = sys.global_cpu_usage();

    CpuInfo { temperature, usage }
}
```

The caller receives a `CpuInfo`. It never sees `sysinfo`. If you later replace `sysinfo` with direct kernel reads (see [sysinfo.md](../crates/sysinfo.md)), this is the only file that changes.

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
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();

    Reading {
        timestamp,
        cpu,
        outdoor_temp: None,
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

pub fn show(reading: &Reading) {
    println!(
        "[{}] CPU: {:.1} °C | Usage: {:.1}%",
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
mod app;
mod domain;
mod infra;
mod interface;

fn main() {
    let reading = app::snapshot::take();
    interface::display::show(&reading);
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

The timestamp is Unix epoch seconds — a plain integer, no external crate needed. The temperature and usage come from the hardware sensors on your machine.

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
