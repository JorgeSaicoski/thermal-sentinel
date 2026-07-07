# Learning Guide

This file is the map. It tells you what to read, in what order, and what to build.

## How this works

An AI agent writes the documentation. You read it and implement the code in `src/`. The gap between a finished doc and working code is intentional — that gap is the exercise. Reading alone is not learning; the compiler is your real teacher.

---

## Step 1 — Learn the language

Read these three documents before writing any code. They are reference material — you will come back to them constantly.

| Document | What it covers |
|---|---|
| [rust/rust_basics.md](rust/rust_basics.md) | Toolchain (`cargo check`, `cargo run`), variables, structs, enums, loops, printing, `#[derive]` |
| [rust/rust_memory.md](rust/rust_memory.md) | Stack vs heap, ownership, borrowing — the concept that makes Rust different from every other language |
| [rust/rust_patterns.md](rust/rust_patterns.md) | `Option`, `Result`, the `?` operator, iterators — how Rust handles missing values and failures |

**After reading:** Run `cargo run` to see the starter program run. Then open `src/main.rs` and read it line by line — it should be short. If any line is unclear, find the concept in the docs above before moving on.

---

## Step 2 — Learn the architecture

Understand how the code is organized and why before building anything.

| Document | What it covers |
|---|---|
| [architecture/project-structure.md](architecture/project-structure.md) | The four-layer structure (`domain`, `infra`, `app`, `interface`), the dependency rule, what goes in each layer |
| [architecture/minimal-startup.md](architecture/minimal-startup.md) | The complete CPU temperature implementation — every file shown in full, then the command to run it |

**Note on minimal-startup.md:** This is the one document in the project that shows complete code for every file — an intentional exception. Seeing the whole picture once helps you understand how the layers connect before you build. Read it all the way through, then close it and build each file from memory. The compiler will tell you what you got wrong.

---

## Step 3 — Build the first feature

**Reference:** [architecture/minimal-startup.md](architecture/minimal-startup.md) and [crates/sysinfo.md](crates/sysinfo.md)

Build the four-layer structure from scratch using CPU temperature as the first feature. After this step, you know the pattern — you know how to take any piece of data, move it through the layers, and display it. Everything from here follows the same shape.

Build the files in this order, compiling after each one:

1. `src/domain/cpu_info.rs` — the `CpuInfo` struct
2. `src/domain/reading.rs` — the `Reading` struct
3. `src/domain/mod.rs` — declare both submodules
4. `src/infra/sensors.rs` — reads hardware using `sysinfo`, returns `CpuInfo`
5. `src/infra/mod.rs` — declare `sensors`
6. `src/app/snapshot.rs` — calls `sensors::read()`, assembles a `Reading`
7. `src/app/mod.rs` — declare `snapshot`
8. `src/interface/display.rs` — formats and prints a `Reading`
9. `src/interface/mod.rs` — declare `display`
10. `src/main.rs` — declare all four modules, call `snapshot::take()` and `display::show()`

**Goal:** `cargo run` prints CPU temperature and usage.

---

## From here, it is your call

Once the CPU layer works, you know how to add any new data source:

- **Domain** — define a type for the data
- **Infra** — write an adapter that fetches or reads it
- **App** — assemble it into your `Reading` (or a new struct, if the shape changes)
- **Interface** — display it

What you add next, and why, is up to you. The tool docs below explain what each library can give you. The ideas docs offer some starting points. Neither tells you what to build.

---

## The tools

Read the relevant doc before using a library. Each one explains what the tool gives you, not what you must build with it.

### [crates/sysinfo.md](crates/sysinfo.md) — hardware and system data

Reads values directly from the operating system. CPU temperature, CPU usage, memory, disk, network, processes. Everything about the machine itself.

Use it when you want to measure something local.

### [crates/reqwest-serde.md](crates/reqwest-serde.md) — HTTP data

Fetches data from any URL and parses JSON into a Rust struct. One pattern works for any API — weather, air quality, exchange rates, anything with a public endpoint.

Use it when you want to bring in data from outside the machine.

### [crates/rusqlite.md](crates/rusqlite.md) — persistent storage

Stores and queries data in a local SQLite file. Insert readings as they happen; query them later to find peaks, averages, trends, or anything SQL can express.

Use it when you want to remember data across runs or compare values over time.

### [crates/clap.md](crates/clap.md) — command-line interface

Adds subcommands, flags, and help text to the tool. Lets the user choose a mode when running the program.

Use it when you want different behaviors accessible from the terminal.

---

## Ideas

Not sure what to build next? These docs are starting points, not instructions.

| Document | What it explores |
|---|---|
| [ideas/what-to-measure.md](ideas/what-to-measure.md) | What `sysinfo` can give you beyond CPU temperature |
| [ideas/data-sources.md](ideas/data-sources.md) | What external data you could fetch with `reqwest` |
| [ideas/cli-modes.md](ideas/cli-modes.md) | Subcommands and arguments you could add (`live`, `save_current`, `analysis`) |
| [ideas/health-score.md](ideas/health-score.md) | Four approaches to a composite health score |

---

## Document index

| Document | One-line description |
|---|---|
| [rust/rust_basics.md](rust/rust_basics.md) | Language reference — syntax, types, structs, enums, control flow |
| [rust/rust_memory.md](rust/rust_memory.md) | Ownership and borrowing — Rust's memory model |
| [rust/rust_patterns.md](rust/rust_patterns.md) | `Option`, `Result`, `?`, iterators |
| [architecture/project-structure.md](architecture/project-structure.md) | Four-layer architecture and the dependency rule |
| [architecture/minimal-startup.md](architecture/minimal-startup.md) | Complete CPU temperature implementation, file by file |
| [crates/sysinfo.md](crates/sysinfo.md) | Reading hardware and system data |
| [crates/reqwest-serde.md](crates/reqwest-serde.md) | HTTP requests and JSON parsing |
| [crates/clap.md](crates/clap.md) | CLI subcommands and flags |
| [crates/rusqlite.md](crates/rusqlite.md) | SQLite database and timestamps |
| [ideas/what-to-measure.md](ideas/what-to-measure.md) | What sysinfo can give you |
| [ideas/data-sources.md](ideas/data-sources.md) | What external APIs you could use |
| [ideas/cli-modes.md](ideas/cli-modes.md) | Subcommands and arguments you could add |
| [ideas/health-score.md](ideas/health-score.md) | Composite health score design approaches |
