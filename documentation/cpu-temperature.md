# Step 1 — Print CPU Temperature

The goal of this step is simple: run `cargo run` and see your CPU temperature printed in the terminal.

By the end you will have used real Rust concepts — crates, traits, iterators, and formatted printing. Each piece is explained below. Your job is to figure out how they fit together.

---

## Before you touch Rust: explore your system

Before writing any code, look at the raw data your machine exposes. Run these in your terminal:

```bash
cat /sys/class/thermal/thermal_zone0/type
cat /sys/class/thermal/thermal_zone0/temp
```

You will see a raw number like `44000`. That is your CPU temperature in **millidegrees Celsius**. Divide by 1000 and you get `44.0 °C`.

This is what `sysinfo` reads under the hood and converts for you.

---

## Piece 1 — What is a crate?

In Rust, external libraries are called **crates**. They are listed in `Cargo.toml` under `[dependencies]`. When you run `cargo build`, Cargo downloads and compiles them automatically.

This project uses [`sysinfo`](https://docs.rs/sysinfo), a crate that reads hardware sensor data.

To add it, open `Cargo.toml` and add one line:

```toml
[dependencies]
sysinfo = "0.33"
```

Run `cargo build` and watch Cargo download it.

---

## Piece 2 — What does `::` mean?

`::` is the **path separator** in Rust. It lets you navigate into a module, a struct, or a crate to reach what you need.

Think of it like folders on your computer:

```
sysinfo::Components
```

means: inside the `sysinfo` crate, find the thing called `Components`.

```
std::thread::sleep
```

means: inside the standard library (`std`), inside the `thread` module, find the function `sleep`.

You will see `::` constantly in Rust. It just means "go deeper into this namespace."

---

## Piece 3 — How to bring things into scope with `use`

Writing the full path every time is verbose. The `use` keyword creates a shortcut:

```rust
use sysinfo::Components;
```

After this line, you can write `Components` instead of `sysinfo::Components` anywhere in the file.

You can bring multiple things from the same crate in one line:

```rust
use sysinfo::{Components, ComponentExt};
```

> `ComponentExt` is a **trait** — explained in the next piece.

---

## Piece 4 — What is a trait?

A trait is a set of methods that a type promises to have. Think of it as an interface or a contract.

`sysinfo` represents each sensor as a **Component**. The methods for reading its data — like getting the label or the temperature — are defined in a trait called `ComponentExt`.

**In Rust, to call a method from a trait, the trait must be in scope.**

This means if you forget to `use sysinfo::ComponentExt`, you will get a compiler error saying the method does not exist — even though it does. Adding the `use` line fixes it.

You do not call `ComponentExt` directly. You just import it and then the methods become available on every `Component` automatically.

The two methods you need:

| Method | Returns | Description |
|--------|---------|-------------|
| `.label()` | `&str` | The sensor name, e.g. `"Core 0"` |
| `.temperature()` | `f32` | Current temperature in °C |

---

## Piece 5 — How to get the list of components

`Components::new_with_refreshed_list()` creates a collection of all hardware sensors detected on your machine, already populated with current readings.

```rust
let components = Components::new_with_refreshed_list();
```

`components` is now a collection. You can iterate over it to access each sensor.

> Tip: hover over `components` in your editor or run `cargo build` — the compiler will tell you the exact type. Getting comfortable reading Rust types is an important skill.

---

## Piece 6 — How to iterate in Rust

To loop over a collection in Rust:

```rust
for item in &collection {
    // item is one element
}
```

The `&` means you are **borrowing** the collection — you read it without taking ownership of it. You will learn more about ownership as you go deeper into Rust. For now: when iterating to just read values, always use `&`.

A concrete example with a vector of numbers:

```rust
let numbers = vec![1, 2, 3];

for n in &numbers {
    println!("{}", n);
}
```

Output:
```
1
2
3
```

---

## Piece 7 — How to print in Rust

Rust uses the `println!` macro to print to the terminal. The `!` means it is a macro, not a regular function.

The first argument is a **format string**. Curly braces `{}` are placeholders that get replaced by the values you pass after the string.

```rust
println!("Hello, {}!", "world");
// Hello, world!

let name = "Jorge";
println!("My name is {}", name);
// My name is Jorge
```

For numbers with decimal places, use `{:.1}` to control how many decimals to show:

```rust
let temp = 44.0_f32;
println!("{:.1}", temp);
// 44.0

println!("{:.2}", temp);
// 44.00
```

You can mix text and multiple values in one `println!`:

```rust
println!("{} is {} years old", "Jorge", 30);
// Jorge is 30 years old
```

---

## Now put the pieces together

You have everything you need:

- How to add a crate (`sysinfo = "0.33"` in `Cargo.toml`)
- What `::` means and how to use `use`
- What a trait is and which trait unlocks the sensor methods
- How to get the list of components
- How to iterate over a collection
- How to print formatted text

Open `src/main.rs` and write a program that prints the label and temperature of every component.

When it works, `cargo run` should output something like:

```
Package id 0: 44.0 °C
Core 0: 43.0 °C
Core 1: 41.0 °C
Core 2: 44.0 °C
Core 3: 42.0 °C
```

---

## Checklist

- [ ] Add `sysinfo` to `Cargo.toml` and run `cargo build`
- [ ] Read the `sysinfo` docs at [docs.rs/sysinfo](https://docs.rs/sysinfo) — find `Components` and `ComponentExt`
- [ ] Write the program in `src/main.rs`
- [ ] Run `cargo run` and see your CPU temperature printed
