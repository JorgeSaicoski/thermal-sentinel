# thermal-sentinel

A CPU temperature monitor — and a project for learning Rust.

## What this is

This project has two purposes: build a working Linux CLI tool, and learn Rust by building it.

An AI agent writes the documentation. You read the docs and write the code. The gap between a finished doc and working code is intentional — that gap is the exercise.

```
documentation/   ← AI writes here (explanations, examples, architecture)
src/             ← You write here (the actual Rust)
```

## What you're building

A Linux CLI tool that:

- Reads CPU temperature from hardware sensors
- Polls at a configurable interval and logs readings to a database
- Fetches outdoor temperature from a public weather API
- Stores everything in SQLite
- Shows a composite health score comparing CPU and ambient conditions

## Prerequisites

Install the Rust toolchain:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or visit [rustup.rs](https://rustup.rs). You need `rustc` and `cargo` available in your terminal.

## Quick start

```bash
git clone <repo>
cd thermal-sentinel
cargo run
```

## How to follow along

Read [`documentation/README.md`](documentation/README.md) — it lays out the six steps in order, tells you which doc to read at each step, and describes what to build after reading it.

Start with `documentation/README.md`, not with the source code.
