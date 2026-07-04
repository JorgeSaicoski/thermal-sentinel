# Minimal Startup — Layers and Contracts

This document uses one concrete example — reading CPU data and wrapping it in a domain type — to explain what each layer is for, what a contract is, and why contracts live in the domain.

Read [project-structure.md](project-structure.md) first if you haven't. This document goes deeper on the *why*.

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

Here is how `CpuInfo` travels from hardware to output, one layer at a time.

### Domain — define the contract

`CpuInfo` is a plain struct with no logic. It just names the two values that describe CPU state at a moment in time.

```rust
pub struct CpuInfo {
    pub temperature: f32,
    pub usage: f32,
}
```

`Reading` holds a `CpuInfo` as a field rather than storing `temperature` and `usage` as separate flat values. This grouping reflects that the two values have the same origin — they come from the same adapter at the same moment.

```rust
pub struct Reading {
    pub timestamp: String,
    pub cpu: CpuInfo,
    pub outdoor_temp: Option<f32>,
    pub city: Option<String>,
}
```

Neither struct imports anything. No `use` statements, no external crates — pure data shapes.

---

### Infra — produce the contract

`sensors` is the only part of the program that knows how to read CPU hardware. It imports `CpuInfo` from the domain and returns one. Everything about `sysinfo` — the `Components` type, the double-refresh pattern, the `Option<f32>` temperature — is hidden inside this layer.

```rust
use crate::domain::cpu_info::CpuInfo;

pub fn read() -> CpuInfo {
    // ... sysinfo code here ...
    CpuInfo { temperature, usage }
}
```

The caller receives a `CpuInfo`. It never sees `sysinfo`. If you later replace `sysinfo` with direct kernel reads (see [sysinfo.md](../crates/sysinfo.md)), this is the only file that changes.

Note the import direction: `infra` imports from `domain`. Never the other way.

---

### Why sensors doesn't return a `Reading`

`sensors` only knows two of the four fields on `Reading`. If it returned a `Reading`, it would have to fill `outdoor_temp: None` and `city: None` — making a claim about weather from inside a hardware adapter. That is not its responsibility.

When you add a weather adapter later, you would have two separate adapters each trying to build an incomplete `Reading`, stepping on each other's fields.

The rule: **each adapter returns what it knows**. The app layer assembles the whole.

---

### App — assemble the whole

`snapshot` is the composition root. It is the only place that calls all adapters and builds a complete `Reading`. It knows about both `CpuInfo` (via the sensors call) and `Reading` (from the domain), but it knows nothing about `sysinfo` or HTTP.

```rust
use crate::domain::reading::Reading;
use crate::infra::sensors;

pub fn take() -> Reading {
    let cpu = sensors::read();   // CpuInfo arrives whole

    Reading {
        timestamp: ...,
        cpu,                     // placed directly — no unpacking
        outdoor_temp: ...,       // filled by weather adapter
        city: ...,
    }
}
```

`cpu` is placed into `Reading` as a unit. No field-by-field unpacking. This is one of the benefits of grouping related values into a named type — the assembly is clean.

---

### Interface — consume without knowing origins

The display layer receives a `Reading` and formats it. It does not know whether temperature came from `sysinfo` or direct kernel reads. It does not know whether the weather fetch succeeded. It just formats what it receives.

```rust
pub fn show(reading: &Reading) {
    println!("CPU: {:.1} °C | Usage: {:.1}%", reading.cpu.temperature, reading.cpu.usage);
}
```

---

## The full flow

```
hardware
  ↓
infra::sensors::read()   →  produces  CpuInfo
                                         ↓
                          app::snapshot::take()   →  produces  Reading
                                                                  ↓
                                               interface::display::show()
```

At every boundary, the only thing that crosses is a domain type. No layer leaks its internals to the next.

---

## What to figure out

The examples above show the *shape* of each piece in isolation. Your exercise is to discover how they connect:

- How does Rust know that `CpuInfo` in `infra/sensors.rs` is the same `CpuInfo` defined in `domain/cpu_info.rs`? (Hint: `use crate::...`)
- How do you make `CpuInfo` visible to other modules? (Hint: `pub`)
- How does each module get registered so Rust knows it exists? (Hint: `mod.rs` and `pub mod`)
- How does `reading.rs` import `CpuInfo` if they're in the same layer?

Read [project-structure.md](project-structure.md) for the module system mechanics, and [rust_memory.md](../rust/rust_memory.md) for why structs are passed by value or by reference depending on context.
