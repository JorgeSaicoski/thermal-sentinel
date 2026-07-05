# Using `clap` — CLI Arguments and Subcommands

`clap` is the standard Rust crate for parsing command-line arguments. With the `derive` feature, you define your CLI entirely through struct and enum annotations — no manual parsing code.

## Add to `Cargo.toml`

```toml
clap = { version = "4", features = ["derive"] }
```

---

## The two types you define

Every `clap` CLI needs two things:

1. A **top-level struct** annotated with `#[derive(Parser)]` — holds global flags and the optional subcommand
2. A **subcommand enum** annotated with `#[derive(Subcommand)]` — each variant is one subcommand

---

## The top-level struct

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "my-tool")]
#[command(about = "A one-line description of what this tool does")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
```

- `#[derive(Parser)]` — generates the parsing logic
- `#[command(name = ...)]` — the program name shown in help text
- `#[command(about = ...)]` — the one-line description shown in help text
- `command: Option<Commands>` — the subcommand is optional. When `None`, the user ran the tool with no subcommand

---

## The subcommand enum

```rust
use clap::Subcommand;

#[derive(Subcommand)]
enum Commands {
    Status,
    Watch {
        #[arg(short, long, default_value_t = 30)]
        interval: u64,
    },
    History {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}
```

Each variant becomes one subcommand. Variants with fields become subcommands with flags:

- `Watch { interval }` → `my-tool watch --interval 5` or `my-tool watch -i 5`
- `History { limit }` → `my-tool history --limit 20` or `my-tool history -l 20`

### `#[arg(...)]` options

| Option | Effect |
|---|---|
| `short` | Generates `-i` from the field name `interval` |
| `long` | Generates `--interval` from the field name |
| `default_value_t = 30` | Default value when the flag is not provided; the `t` means "typed" |

---

## Parsing in `main`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        None                                => run_default()?,
        Some(Commands::Status)              => run_status()?,
        Some(Commands::Watch { interval })  => run_watch(interval)?,
        Some(Commands::History { limit })   => run_history(limit)?,
    }

    Ok(())
}
```

`Cli::parse()` reads `std::env::args()` and populates the struct. If the user types something invalid, `clap` prints an error and exits before `parse()` returns — you do not handle that.

Each `run_*` function holds the logic for one mode. The function signature receives the subcommand's fields as plain arguments.

---

## Free help text

Once `clap` is set up, `--help` works automatically at every level:

```
my-tool --help
my-tool watch --help
my-tool history --help
```

`clap` generates the help text from your struct and field names — you do not write it.

---

## Why `Option<Commands>` and not just `Commands`

If `command` were a plain `Commands` (not `Option`), running the tool with no subcommand would be an error. Wrapping it in `Option` makes the subcommand optional — `None` becomes the default behavior.

---

## The `--` separator in `cargo run`

When passing subcommand arguments through `cargo run`, you need `--` to separate cargo's own arguments from your program's arguments:

```bash
cargo run -- watch --interval 5
cargo run -- history -l 20
cargo run -- --help
```

When running the compiled binary directly, `--` is not needed:

```bash
./target/debug/my-tool watch --interval 5
```

---

## Adding flags to `colored` output

The `colored` crate integrates well alongside `clap`. If you want a `--no-color` flag:

```rust
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    no_color: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}
```

Then check `cli.no_color` before printing colored output.

---

## Further reading

- [clap docs — derive tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [rust_basics.md](../rust/rust_basics.md) — enums and match, which you use to dispatch on Commands
- [rust_patterns.md](../rust/rust_patterns.md) — `Result` and `?` in your `run_*` functions
- [ideas/cli-modes.md](../ideas/cli-modes.md) — modes and subcommands you could add
