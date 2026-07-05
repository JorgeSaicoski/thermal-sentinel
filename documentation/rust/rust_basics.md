# Rust Basics — Language, Types, and Tooling

This is the reference doc for Rust the language. It covers syntax, types, control flow, and the tools you use every day. Read it before starting any step.

---

## The toolchain

Three commands cover almost everything:

```bash
cargo check   # type-checks your code instantly — no binary produced
cargo build   # compiles a binary in target/debug/
cargo run     # compiles and runs
```

Use `cargo check` constantly while writing. It is much faster than `cargo build` and catches all type and borrow errors. Only reach for `cargo run` when you want to see the program actually run.

---

## Reading compiler errors

Rust's compiler error messages are designed to be read completely, not just scanned. They have a consistent structure:

```
error[E0382]: borrow of moved value: `label`
  --> src/main.rs:12:20
   |
9  |     let other = label;
   |                 ----- value moved here
12 |     println!("{}", label);
   |                    ^^^^^ value borrowed here after move
   |
help: consider cloning the value if the performance cost is acceptable
```

- `error[E0382]` — the error code. Run `rustc --explain E0382` for a detailed explanation with examples.
- `-->` — the exact file and line.
- `help:` — often the exact fix. Read this first.

**Common errors:**

| Code | Message | Likely cause |
|---|---|---|
| E0382 | use of moved value | Used a value after giving ownership away |
| E0502 | conflicting borrows | Have `&` and `&mut` borrow active at the same time |
| E0597 | does not live long enough | Reference outlives the value it points to |
| E0308 | type mismatch | Passed `f64` where `f32` expected — cast with `as` |
| E0277 | trait not implemented | Tried `{}` on a type without `Display` — use `{:?}` |

---

## Crates and modules

External libraries are called **crates**. Declare them in `Cargo.toml`:

```toml
[dependencies]
sysinfo = "0.33"
```

`cargo build` downloads and compiles them. Version `"0.33"` means any compatible `0.33.x`.

### `::` — the path separator

`::` navigates into a namespace:

```rust
sysinfo::Components         // inside sysinfo, find Components
std::thread::sleep          // inside std, inside thread, find sleep
```

### `use` — bring names into scope

```rust
use sysinfo::{Components, Component};

// now write Components instead of sysinfo::Components
```

If the same name is imported twice, the compiler refuses — names must be unique in scope.

---

## Variables

```rust
let x = 5;           // immutable — cannot be reassigned
let mut y = 5;       // mutable — can be changed
y += 1;

const MAX_TEMP: f32 = 100.0;   // compile-time constant, must have type
```

Rust infers types from context. You can annotate explicitly:

```rust
let temp: f32 = 44.0;
let count: usize = 0;
```

Common numeric types:

| Type | What it is |
|---|---|
| `i32`, `i64` | Signed integer (negative and positive) |
| `u32`, `u64`, `usize` | Unsigned integer (non-negative) |
| `f32`, `f64` | Floating-point number |
| `bool` | `true` or `false` |

Rust does **not** automatically convert between numeric types. To convert: `value as f32`, `value as i64`, etc.

---

## Printing

```rust
println!("hello");                        // newline included
println!("temp: {}", 72.5);              // {} for Display types
println!("temp: {:.1}", 72.5);           // one decimal place → 72.5
println!("delta: {:+.1}", 5.3);          // force sign → +5.3
println!("{:<20} {:>8}", "label", 72.5); // left-align 20, right-align 8
println!("{:?}", some_vec);              // Debug format — works on most types
println!("{:#?}", complex_struct);       // pretty-printed Debug
eprintln!("warning: {}", msg);           // to stderr instead of stdout
```

`println!` is a **macro** (the `!` means it is not a regular function). `format!` works the same way but returns a `String` instead of printing.

### `dbg!` — print and return

```rust
let y = dbg!(x * 2) + 1;
// [src/main.rs:5] x * 2 = 10
// y is 11
```

Prints the expression, its value, and the source location — then returns the value so you can use it inline. Prints to stderr. Remove before committing.

---

## Structs — grouping named data

```rust
struct Sensor {
    label: String,
    temperature: f32,
    active: bool,
}
```

Create and use:

```rust
let s = Sensor {
    label: String::from("Core 0"),
    temperature: 62.5,
    active: true,
};

println!("{}: {:.1} °C", s.label, s.temperature);
```

To modify a field, the binding must be `mut`:

```rust
let mut s = Sensor { ... };
s.temperature = 71.0;
```

`pub` on the struct and its fields makes them accessible from other modules:

```rust
pub struct Sensor {
    pub label: String,
    pub temperature: f32,
}
```

---

## Enums — a value with multiple possible shapes

```rust
enum Status {
    Cool,
    Warm,
    Hot,
}

let current = Status::Warm;
```

Variants can carry data — each variant independently:

```rust
enum Reading {
    Available(f32),
    Unavailable,
    Error(String),
}

let r = Reading::Available(62.5);
```

---

## `impl` — attaching behavior to a type

```rust
impl Sensor {
    // associated function — called with ::
    fn new(label: &str, temp: f32) -> Sensor {
        Sensor { label: String::from(label), temperature: temp, active: true }
    }

    // method — called with . on an instance
    fn is_overheating(&self) -> bool {
        self.temperature > 90.0
    }

    // mutable method — can change the struct
    fn set_temp(&mut self, temp: f32) {
        self.temperature = temp;
    }
}

let mut s = Sensor::new("Core 0", 62.5);
println!("{}", s.is_overheating());   // false
s.set_temp(95.0);
println!("{}", s.is_overheating());   // true
```

---

## `match` — exhaustive pattern matching

```rust
match current {
    Status::Cool => println!("All good"),
    Status::Warm => println!("Getting warm"),
    Status::Hot  => println!("Critical"),
}
```

Every variant must be handled — the compiler rejects incomplete matches. Extract data from variants:

```rust
match r {
    Reading::Available(temp) => println!("{:.1} °C", temp),
    Reading::Unavailable     => println!("no data"),
    Reading::Error(msg)      => println!("error: {}", msg),
}
```

`match` is an expression — it returns a value:

```rust
let label = match current {
    Status::Cool => "COOL",
    Status::Warm => "WARM",
    Status::Hot  => "HOT",
};
```

Use `_` to catch unhandled variants (use sparingly — it hides new variants):

```rust
match current {
    Status::Hot => println!("Critical!"),
    _           => println!("OK"),
}
```

### `if let` — match one variant

```rust
if let Reading::Available(temp) = r {
    println!("{:.1} °C", temp);
}
// equivalent to match with a _ => {} fallthrough
```

---

## `#[derive]` — automatic trait implementations

```rust
#[derive(Debug, Clone, PartialEq)]
struct Sensor {
    label: String,
    temperature: f32,
}
```

| Derive | What you get |
|---|---|
| `Debug` | `{:?}` and `{:#?}` formatting |
| `Clone` | `.clone()` — independent copy |
| `PartialEq` | `==` and `!=` comparisons |
| `Copy` | Automatic copy on assignment (only for stack-only types) |

Add `Debug` to every type you define — it is free and essential for debugging.

---

## Traits — shared behavior

A **trait** is a set of methods a type promises to provide. `Display` means a type knows how to format itself with `{}`. `f32` has it; `Option<f32>` does not — which is why you get a compile error when you try to print an `Option` with `{}`.

You do not call traits directly. Import the trait, and its methods become available on types that implement it.

---

## Control flow

### `if` / `else` — also an expression

```rust
let status = if temp > 90.0 { "critical" } else { "normal" };
```

No parentheses around the condition.

### `loop` — infinite, break to exit

```rust
loop {
    // do work
    if done { break; }
}

// break can return a value
let result = loop {
    let v = read_sensor();
    if v > 0.0 { break v; }
};
```

### `while` — condition before each iteration

```rust
while count < 5 {
    count += 1;
}
```

### `for` — over a collection or range

```rust
for sensor in &sensors {      // & borrows — collection stays usable after
    println!("{}", sensor);
}

for i in 0..5 { }    // 0, 1, 2, 3, 4
for i in 0..=5 { }   // 0, 1, 2, 3, 4, 5

for (i, item) in sensors.iter().enumerate() { }
```

### `break`, `continue`, labels

```rust
'outer: for i in 0..5 {
    for j in 0..5 {
        if i + j > 6 { break 'outer; }
    }
}
```

### Pausing — `thread::sleep`

```rust
use std::thread;
use std::time::Duration;

thread::sleep(Duration::from_secs(30));
thread::sleep(Duration::from_millis(500));
```

`Duration` keeps units explicit — you cannot pass seconds where milliseconds are expected.

### The polling loop

Combining `loop` and `thread::sleep` gives the skeleton of any monitor or watcher:

```rust
use std::thread;
use std::time::Duration;

loop {
    let reading = take_reading();    // do work
    display(reading);                // show or save

    thread::sleep(Duration::from_secs(30));  // wait, then repeat
}
```

The sleep goes at the end — so the first reading appears immediately on startup. The loop runs until the user presses `Ctrl+C`.

---

## `RUST_BACKTRACE=1`

When a panic occurs (`.unwrap()` on `None`, index out of bounds, etc.), get a full stack trace:

```bash
RUST_BACKTRACE=1 cargo run
```

Look for your own code in the output — the standard library frames at the top are usually not relevant.

---

## Further reading

- [The Rust Book](https://doc.rust-lang.org/book/) — free online, the authoritative Rust reference
- [rust_memory.md](rust_memory.md) — ownership, borrowing, stack vs heap
- [rust_patterns.md](rust_patterns.md) — Result, iterators, error handling
