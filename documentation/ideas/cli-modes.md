# CLI Modes — Ideas

Once your tool does one thing, adding subcommands lets the user choose what to do when they run it. This document shows some ways to think about modes, what arguments each might take, and how they connect to code.

Read [crates/clap.md](../crates/clap.md) when you're ready to implement.

---

## Thinking about modes

Before naming a command, ask: what question does running this answer?

- `thermal-sentinel` alone → "What is my system doing right now?"
- `thermal-sentinel live` → "Show me what is happening as it changes"
- `thermal-sentinel save_current` → "Record this moment permanently"
- `thermal-sentinel analysis` → "What have I seen over time?"

Each is a distinct intention. They might call different app functions, accept different arguments, and produce different output — but they all share the same underlying data.

---

## Some modes to consider

### No subcommand — the default

Running the tool with no arguments prints one reading and exits. This is always useful as a sanity check, even if you add many other modes.

```
thermal-sentinel
```

### live

Shows readings continuously, updating as data changes.

```
thermal-sentinel live
thermal-sentinel live --interval 5
```

What it does: takes a reading, displays it, sleeps, repeats until interrupted with `Ctrl+C`.

What argument makes sense: `--interval` — how many seconds between updates.

### save_current

Takes one reading and saves it to storage, then exits.

```
thermal-sentinel save_current
```

What it does: one reading, one insert, done. No loop.

No required arguments — or optionally a `--note` flag to attach a label to that reading.

### analysis

Looks at past readings and shows something derived from them.

```
thermal-sentinel analysis
thermal-sentinel analysis --last 20
thermal-sentinel analysis --peak
```

What it does: queries storage, computes something (average, max, trends), displays it.

Arguments to consider: `--last N` for the N most recent readings, `--peak` to show only the maximum value recorded, `--today` to filter by date.

### compare

Shows two readings side by side — or the change between them.

```
thermal-sentinel compare --morning --now
```

This one is open-ended. "Compare" could mean comparing two time points, comparing two sensors, or comparing CPU temperature against outdoor temperature. What you compare depends on what data you have.

---

## How modes map to code

Each subcommand routes to one function in the app layer. The interface layer parses the command; the app layer does the work.

```
thermal-sentinel             → app::snapshot::show()
thermal-sentinel live        → app::watch::run(interval)
thermal-sentinel save_current → app::snapshot::save()
thermal-sentinel analysis    → app::history::run(limit)
```

The `match` in `main.rs` connects them:

```rust
match cli.command {
    None                                    => app::snapshot::show(),
    Some(Commands::Live { interval })       => app::watch::run(interval),
    Some(Commands::SaveCurrent)             => app::snapshot::save(),
    Some(Commands::Analysis { limit, peak }) => app::history::run(limit, peak),
}
```

The interface layer dispatches. It never does the work itself.

---

## Arguments vs flags

A value the user provides is an argument. Something the user either enables or doesn't is a flag.

```rust
#[derive(Subcommand)]
enum Commands {
    Live {
        #[arg(short, long, default_value_t = 30)]
        interval: u64,        // --interval 5  (a value)
    },
    Analysis {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,         // --limit 50    (a value)
        #[arg(long)]
        peak: bool,           // --peak        (a flag, no value)
    },
}
```

`default_value_t` means the argument is optional — if the user does not provide it, they get the default. A `bool` flag with no default is `false` when absent and `true` when present.

---

## Naming conventions

Enum variant names in Rust become command names on the terminal. Clap converts `PascalCase` variants to `kebab-case` automatically:

| Enum variant | Command the user types |
|---|---|
| `SaveCurrent` | `save-current` |
| `Live` | `live` |
| `Analysis` | `analysis` |

Pick a style — verb-based (`save`, `watch`, `compare`) or noun-based (`live`, `history`, `snapshot`) — and stay consistent.

---

## Further reading

- [crates/clap.md](../crates/clap.md) — the full implementation: struct, enum, `#[arg]`, parsing in main
- [rust/rust_basics.md](../rust/rust_basics.md) — `enum` and `match`, which dispatch commands
- [rust/rust_patterns.md](../rust/rust_patterns.md) — `Result` and `?` for the functions each command calls
