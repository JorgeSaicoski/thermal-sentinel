# thermal-sentinel

A CPU temperature monitoring tool — and a project for learning Rust.

## Purpose

This project exists to learn Rust through practice. The goal is not just to build a working tool, but to understand *why* each piece of code is written the way it is.

## How this project works

**AI agents** (like Claude) have one role here: **teaching through documentation**. They write `.md` files in the `documentation/` folder and open PRs — never source code directly.

**You** (the human) read the documentation, understand the concepts, and implement the code yourself in the `src/` folder.

```
documentation/   ← AI writes here (explanations, examples, concepts)
src/             ← You write here (the actual Rust implementation)
```

This means you will sometimes see a doc that explains exactly how something works, with annotated code examples, but no corresponding implementation yet. That gap is intentional — it is your exercise.

## Why this approach

Reading documentation and then writing the code yourself is one of the fastest ways to build real understanding. If the AI writes the code for you, you get a working program but you don't learn Rust. If the AI explains the concept and you implement it, you get both.

## Project goal

Build a cross-platform (Linux) CLI tool that:

- Reads CPU temperature from system hardware sensors
- Polls at a configurable interval
- Logs readings to a file (CSV)
- Warns when temperature exceeds a threshold

## Documentation index

| Document | Topic |
|----------|-------|
| [cpu-temperature.md](documentation/cpu-temperature.md) | How Linux exposes CPU temps and how to read them in Rust |

## Getting started

```bash
git clone <repo>
cd thermal-sentinel
cargo run
```

Prerequisites: Rust toolchain via [rustup.rs](https://rustup.rs).
