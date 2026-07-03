# thermal-sentinel

A CPU temperature monitoring tool — and a project for learning Rust.

## Purpose

This project exists to learn Rust through practice. The goal is not just to build a working tool, but to understand *why* each piece of code is written the way it is.

## How this project works

**AI agents** (like Claude) have one role here: **teaching through documentation**. They write `.md` files in the `documentation/` folder — never source code.

**You** (the human) read the documentation, understand the concepts, and implement the code yourself in the `src/` folder.

```
documentation/   ← AI writes here (explanations, examples, concepts)
src/             ← You write here (the actual Rust implementation)
```

The gap between a doc and working code is intentional — that gap is the exercise.

## Project goal

Build a Linux CLI tool that:

- Reads CPU temperature from hardware sensors
- Polls at a configurable interval
- Compares with outdoor temperature via public weather APIs
- Logs readings to a SQLite database
- Shows a composite health score

## Documentation

### Rust concepts

Read these before the build steps. They explain the language, not the project.

| Document | What it covers |
|---|---|
| [rust_basics.md](documentation/rust_basics.md) | Toolchain, variables, structs, enums, match, control flow, printing, debugging |
| [rust_memory.md](documentation/rust_memory.md) | Stack vs heap, ownership, borrowing, lifetimes |
| [rust_patterns.md](documentation/rust_patterns.md) | `Result`, `Option`, `?`, iterators, `filter_map`, `max_by` |

### Crate guides

One doc per external library. Read the relevant crate doc before each build step.

| Document | What it covers |
|---|---|
| [sysinfo.md](documentation/sysinfo.md) | Reading CPU temperature and usage |
| [reqwest-serde.md](documentation/reqwest-serde.md) | HTTP requests and JSON parsing |
| [clap.md](documentation/clap.md) | CLI subcommands and flags |
| [rusqlite.md](documentation/rusqlite.md) | SQLite database and timestamps with `chrono` |

### Architecture

| Document | What it covers |
|---|---|
| [project-structure.md](documentation/project-structure.md) | Splitting code into `domain/`, `app/`, `infra/`, `interface/` |

### Build steps

| Document | What you build |
|---|---|
| [steps.md](documentation/steps.md) | All eight exercises in order, with checklists |

## Getting started

```bash
git clone <repo>
cd thermal-sentinel
cargo run
```

Prerequisites: Rust toolchain via [rustup.rs](https://rustup.rs).

Start with [rust_basics.md](documentation/rust_basics.md), then follow [steps.md](documentation/steps.md).
