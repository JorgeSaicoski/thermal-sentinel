# thermal-sentinel

A system monitoring tool — and a project for learning Rust.

## What this is

This project has two purposes: build a Linux CLI tool that monitors your machine, and learn Rust by building it.

An AI agent writes the documentation. You read the docs and write the code. The gap between a finished doc and working code is intentional — that gap is the exercise.

```
documentation/   ← AI writes here (explanations, examples, architecture)
src/             ← You write here (the actual Rust)
```

## What you're building

The first feature — reading CPU temperature through a four-layer architecture — is documented in full. What you add after that is yours to decide.

Some directions: log readings over time, fetch data from an external API, add CLI subcommands, compare values across days or conditions. The documentation gives you the tools; you design the application.

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

Read [`documentation/README.md`](documentation/README.md). It walks you through the language basics, the architecture, and the first implementation — then hands the project over to you.
