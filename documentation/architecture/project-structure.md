# Project Structure — Modules and Clean Architecture

Right now everything lives in `src/main.rs`. As the program grows across five modes with HTTP requests, a database, hardware sensors, and formatted output, that single file becomes impossible to navigate.

This document covers two things:

1. **How Rust modules work** — how to split code across files and folders
2. **How to apply clean architecture** — which code goes in which layer and why

---

## The target structure

```
src/
├── main.rs              ← entry point only — parses args, dispatches
│
├── domain/              ← pure business logic, no external crates
│   ├── mod.rs
│   ├── cpu_info.rs      ← CpuInfo struct (temperature + usage as one unit)
│   ├── reading.rs       ← the Reading struct (one full row of data)
│   └── score.rs         ← health score formula
│
├── app/                 ← use cases — orchestrate the other layers
│   ├── mod.rs
│   ├── snapshot.rs      ← take one current reading
│   ├── watch.rs         ← poll on a loop and save readings
│   └── history.rs       ← retrieve past readings
│
├── infra/               ← external systems: hardware, network, database
│   ├── mod.rs
│   ├── sensors.rs       ← reads CPU temp and usage via sysinfo
│   ├── weather.rs       ← fetches city and outdoor temp via HTTP
│   └── db.rs            ← SQLite: open, create schema, insert, query
│
└── interface/           ← everything the user sees and types
    ├── mod.rs
    ├── cli.rs           ← clap Cli struct and Commands enum
    └── display.rs       ← all formatting and println! calls
```

---

## How Rust modules work

### Declaring a module

In Rust, a folder is not automatically a module. You must declare it explicitly.

In `src/main.rs`, declare each top-level module:

```rust
mod domain;
mod app;
mod infra;
mod interface;

fn main() { ... }
```

For each `mod name;` declaration, Rust looks for either:
- `src/name.rs` — a single file module
- `src/name/mod.rs` — a folder module with submodules

We are using folders, so each layer needs a `mod.rs` file inside it.

### `mod.rs` — the entry point for a folder

`src/domain/mod.rs` declares what is inside the `domain` module:

```rust
pub mod reading;
pub mod score;
```

`pub mod` means the submodule is visible to code outside `domain`. Without `pub`, the submodule exists but nothing can use it from the outside.

### `pub` — controlling visibility

By default, everything in Rust is private — only visible inside the same module. Use `pub` to make things accessible from other modules:

```rust
// src/domain/reading.rs

pub struct Reading {          // pub: other modules can use this type
    pub timestamp: String,    // pub: other modules can read this field
    pub cpu_temp: f32,
    pub cpu_usage: f32,
    pub outdoor_temp: Option<f32>,
    pub city: Option<String>,
}
```

If you forget `pub` on a struct field, Rust will tell you:
```
error[E0616]: field `cpu_temp` of struct `Reading` is private
```

### Importing from another module

Use `use crate::...` to import from elsewhere in your own crate:

```rust
// src/app/snapshot.rs

use crate::domain::reading::Reading;
use crate::infra::sensors;
use crate::infra::weather;
```

`crate` always refers to the root of your own project. Think of it as the starting point of every path inside your codebase.

### Re-exporting for convenience

`mod.rs` files can re-export things to simplify imports for callers:

```rust
// src/domain/mod.rs

pub mod reading;
pub mod score;

pub use reading::Reading;      // callers can write `use crate::domain::Reading`
pub use score::HealthScore;    // instead of `use crate::domain::reading::Reading`
```

This is optional but makes imports in other layers cleaner.

---

## The four layers

### Layer 1 — `domain/`

The domain layer contains your core data types and business rules. It has **no external crate dependencies** — only the Rust standard library.

This is the most important constraint. If `domain/` imports `sysinfo`, `rusqlite`, or `reqwest`, the entire point of the layer is lost. Domain logic should be testable without a database, without a network, without hardware.

**What goes here:**

`cpu_info.rs` — the `CpuInfo` struct. Groups temperature and usage into one named unit rather than leaving them as separate flat fields on `Reading`. This makes `Reading` easier to assemble — the app layer gets a complete `CpuInfo` from `sensors.rs` and places it whole:

```rust
pub struct CpuInfo {
    pub temperature: f32,
    pub usage: f32,
}
```

`reading.rs` — the `Reading` struct. One reading is one full row of data:

```rust
pub struct Reading {
    pub timestamp: String,
    pub cpu: CpuInfo,
    pub outdoor_temp: Option<f32>,
    pub city: Option<String>,
}
```

Why `pub cpu: CpuInfo` and not flat `cpu_temp`/`cpu_usage` fields? Because those two values have the same origin — they come from the same infra adapter at the same moment. Grouping them reflects that. It also means `sensors.rs` can return a `CpuInfo` that carries both values together, and the caller doesn't have to unpack a tuple or remember which position is which.

`score.rs` — the health score formula. Takes three `f32` values: hottest CPU temperature, average usage, and external temperature. Returns a score. No external crates, no IO — pure domain logic. See [ideas/health-score.md](../ideas/health-score.md) for the formula and the full architecture.

### Layer 2 — `infra/`

The infra layer talks to the outside world: hardware, network, and disk. Each file wraps one external dependency.

**What goes here:**

`sensors.rs` — wraps `sysinfo`. Imports `CpuInfo` from the domain layer and returns one. Nothing in this file knows about the weather, the database, or the full `Reading` struct.

Why return `CpuInfo` and not `Reading`? Because `sensors.rs` can only fill two of `Reading`'s four fields. If it returned a `Reading`, it would have to write `outdoor_temp: None, city: None` — making a domain-level decision ("there is no weather data") from inside an infra adapter that has no business making that call. When you add `weather.rs` later, you would have two adapters each building an incomplete `Reading` and stepping on each other. The right pattern: each adapter returns what it knows, the app layer assembles the whole.

`sensors.rs` is also the **only** file in the codebase that imports `sysinfo`. If you ever want to swap it for direct `hwmon` reads (see [sysinfo.md](../crates/sysinfo.md)), you change one file.

`weather.rs` — wraps `reqwest` + `serde`. Fetches location and outdoor temperature. Returns a plain `(String, f32)` tuple or an error — the domain types, not raw JSON.

`db.rs` — wraps `rusqlite`. Opens the database, creates the schema if needed, inserts a `Reading`, queries a `Vec<Reading>`. The domain `Reading` type is what goes in and comes out — no raw SQL types leak into the rest of the program.

### Layer 3 — `app/`

The app layer contains **use cases** — it orchestrates the other layers without knowing how they work. It calls `infra` to get data, builds domain types, and returns results. It never formats output.

**What goes here:**

`snapshot.rs` — calls `sensors` and `weather`, assembles a `Reading`, returns it. This is the only place that knows about all sources and has the authority to build the complete domain object.

```rust
pub fn take_snapshot() -> Result<Reading, Box<dyn std::error::Error>> {
    let cpu = sensors::read()?;            // returns CpuInfo
    let weather = weather::fetch().ok();   // None if network fails

    Ok(Reading {
        timestamp: ...,
        cpu,                               // CpuInfo placed whole — no unpacking
        outdoor_temp: weather.map(|w| w.temp),
        city: weather.map(|w| w.city),
    })
}
```

`watch.rs` — runs the polling loop: calls `snapshot::take_snapshot()`, calls `db::insert()`, sleeps.

`history.rs` — calls `db::query(limit)`, returns `Vec<Reading>`.

### Layer 4 — `interface/`

The interface layer is everything the user interacts with. It knows about the terminal but nothing about the database or HTTP.

**What goes here:**

`cli.rs` — the `clap` `Cli` struct and `Commands` enum. Exactly as documented in `cli-structure.md`, but now in its own file.

`display.rs` — all `println!` calls, all color formatting, all table alignment. Every mode's output logic lives here as a function:

```rust
pub fn show_snapshot(reading: &Reading) { ... }
pub fn show_peak(label: &str, temp: f32) { ... }
pub fn show_indicator(score: &HealthScore, reading: &Reading) { ... }
pub fn show_history(readings: &[Reading]) { ... }
```

---

## The dependency rule

The layers can only depend inward:

```
interface  →  app  →  infra  →  domain
                  ↘           ↗
                   domain ←──
```

Concretely:
- `domain` imports nothing from your own crate
- `infra` can import from `domain`
- `app` can import from `domain` and `infra`
- `interface` can import from `domain`, `infra`, and `app`
- `main.rs` can import from all layers

**What this prevents:**

- `domain` should never import `rusqlite` — business rules should not depend on how you store data
- `infra` should never import `clap` — a database function should not know what a CLI flag is
- `app` should never call `println!` — a use case should not know what a terminal is

If you find yourself wanting to break one of these rules, it usually means the code belongs in a different layer.

---

## What `main.rs` looks like when the layers are in place

Once all four layers exist, `main.rs` becomes minimal:

```rust
mod domain;
mod app;
mod infra;
mod interface;

use interface::cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        None                                => app::snapshot::run_default()?,
        Some(Commands::Peak)                => app::snapshot::run_peak()?,
        Some(Commands::Indicator)           => app::snapshot::run_indicator()?,
        Some(Commands::Watch { interval })  => app::watch::run(interval)?,
        Some(Commands::History { limit })   => app::history::run(limit)?,
    }

    Ok(())
}
```

It declares the modules, parses the CLI, and dispatches. Nothing else.

---

## How to build the layers

Do this in small steps — add one layer at a time and compile after each step. Trying to create everything at once makes compiler errors hard to trace.

**Suggested order:**

1. Create `src/domain/mod.rs`, `reading.rs`, `score.rs`. Declare the domain types. Fix imports in `main.rs`.
2. Create `src/infra/mod.rs`, `sensors.rs`, `weather.rs`, `db.rs`. Add the data-fetching adapters.
3. Create `src/app/mod.rs`, `snapshot.rs`, `watch.rs`, `history.rs`. Add the use case logic.
4. Create `src/interface/mod.rs`, `cli.rs`, `display.rs`. Add the `Cli` struct and all `println!` calls.
5. Slim down `main.rs` to just module declarations and dispatch.

After each step, run `cargo build`. Fix errors before moving to the next step. The compiler will tell you exactly which imports are missing or which items need `pub`.

---

## Your task

Build the layer structure from scratch. Start from a working `main.rs` that reads CPU data directly, then add each layer one file at a time. Compile after each new file — fix errors before moving to the next.

A small change that compiles is better than a large change that doesn't. When the build breaks, you know exactly which file caused it.

---

## Checklist

- [ ] Create `src/domain/mod.rs` — declare `pub mod cpu_info; pub mod reading;`
- [ ] Create `src/domain/cpu_info.rs` — `CpuInfo` struct with `temperature: f32` and `usage: f32`
- [ ] Create `src/domain/reading.rs` — `Reading` struct with `pub cpu: CpuInfo` and all other `pub` fields
- [ ] Create `src/infra/mod.rs` — declare `pub mod sensors;`
- [ ] Create `src/infra/sensors.rs` — wraps `sysinfo`, returns `CpuInfo`
- [ ] Create `src/app/mod.rs` — declare `pub mod snapshot;`
- [ ] Create `src/app/snapshot.rs` — calls adapters, assembles a complete `Reading`
- [ ] Create `src/interface/mod.rs` — declare `pub mod display;`
- [ ] Create `src/interface/display.rs` — all `println!` calls and output formatting
- [ ] Write `src/main.rs` — declare all four modules, call `snapshot::take()` and `display::show()`
- [ ] Run `cargo run` — prints CPU temperature and usage
- [ ] Add `src/infra/weather.rs` — fetches city and outdoor temp from an HTTP API
- [ ] Add `src/infra/db.rs` — opens the database, creates the schema, inserts and queries readings
- [ ] Add `src/app/watch.rs` — polling loop: take snapshot, save to db, sleep
- [ ] Add `src/app/history.rs` — queries the database, returns a `Vec<Reading>`
- [ ] Add `src/interface/cli.rs` — the `Cli` struct and `Commands` enum
- [ ] Update `src/main.rs` — parse CLI, dispatch on subcommand
- [ ] Run `cargo run -- --help` — usage menu appears

---

## Further reading

- [The Rust Book, Chapter 7 — Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [clap.md](../crates/clap.md) — the `Cli` and `Commands` types that move to `interface/cli.rs`
- [rust_memory.md](../rust/rust_memory.md) — ownership patterns you will encounter passing `Reading` values between layers
- [minimal-startup.md](minimal-startup.md) — a concrete walkthrough of the layers using `CpuInfo` as the example
