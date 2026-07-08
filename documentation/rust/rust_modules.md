# Rust Modules — `mod`, `pub`, and `use`

This document explains how Rust organizes code across files and folders, and how `mod.rs` fits into that. Read this before working on any multi-file part of the project.

---

## The mental model

Most languages handle visibility and imports in one step. Rust separates them into three distinct things:

| What you want | Rust keyword |
|---|---|
| Tell Rust a file exists | `mod filename;` |
| Make something visible outside its file | `pub` |
| Bring a name into local scope | `use path::Name;` |

Each of these is a separate keyword. You need all three to use something from another file.

---

## `mod` — declaring a module exists

Rust does not scan your filesystem automatically. You must tell it about each file.

If you have a file called `reading.rs`, nothing uses it unless someone writes:

```rust
mod reading;
```

That line says: "there is a module named `reading`, and it lives in `reading.rs`." Without it, the file is invisible to the compiler.

This is different from Go, where any `.go` file in a folder is automatically part of the package.

---

## `pub` — making something visible

By default, everything in Rust is private. Private means: only code in the same module can see it.

To make a function visible outside its module:

```rust
pub fn describe() -> String {
    String::from("Core 0: 62.5 °C")
}
```

To make a struct visible:

```rust
pub struct Reading {
    pub label: String,
    pub temperature: f32,
}
```

Notice `pub` appears twice — once on the struct, and once on each field. The struct being public does not make its fields public. If you forget `pub` on a field, the compiler will let you name the type but not read any data from it.

---

## `use` — bringing a name into local scope

`use` is a shortcut. Without it, you write the full path every time:

```rust
let r = domain::reading::Reading { ... };
```

With `use`:

```rust
use domain::reading::Reading;

let r = Reading { ... };
```

`use` does not import a file or make anything public. It just lets you write a shorter name. The item has to already be public (via `pub`) for `use` to work.

---

## `mod.rs` — the entry point for a folder

When a module lives in a folder instead of a single file, Rust looks for `mod.rs` inside that folder as the entry point.

Example structure:

```
src/
  domain/
    mod.rs
    reading.rs
    cpu_info.rs
```

`domain/mod.rs` is the entry point for the `domain` module. It is responsible for declaring what submodules exist:

```rust
// domain/mod.rs
pub mod reading;
pub mod cpu_info;
```

That is it. Those two lines tell Rust: "the `domain` module contains two submodules, and both are public."

`mod.rs` does not export a function. It does not export a file. It declares which submodules belong to this folder-module.

---

## Re-exporting from `mod.rs`

You can also use `mod.rs` to make a submodule's item look like it belongs directly to the parent module. This is called **re-exporting**:

```rust
// domain/mod.rs
pub mod reading;
pub use reading::Reading;
```

Without re-export, callers write:

```rust
use domain::reading::Reading;
```

With `pub use` in `mod.rs`, callers can write:

```rust
use domain::Reading;
```

Both work. The difference is how deep the caller has to reach. Re-exporting is a design choice — it hides the internal structure and gives callers a cleaner path.

---

## How the pieces connect

Here is a complete example showing all three keywords working together:

```
src/
  main.rs
  sensors/
    mod.rs
    temperature.rs
```

```rust
// sensors/temperature.rs
pub struct Reading {
    pub label: String,
    pub value: f32,
}
```

```rust
// sensors/mod.rs
pub mod temperature;
pub use temperature::Reading;   // optional re-export
```

```rust
// main.rs
mod sensors;                    // tell Rust the sensors/ folder exists
use sensors::Reading;           // works because of the re-export in mod.rs

fn main() {
    let r = Reading { label: String::from("Core 0"), value: 62.5 };
    println!("{}: {}", r.label, r.value);
}
```

Without the `mod sensors;` line in `main.rs`, none of this compiles — Rust would not know the folder exists.

---

## Common errors and what they mean

| Error | What happened |
|---|---|
| `cannot find type Reading in this scope` | Missing `use` — bring the name into scope |
| `module reading is private` | The `mod` in `mod.rs` is missing `pub` |
| `field temperature is private` | The struct field is missing `pub` |
| `unresolved import domain::Reading` | Missing `pub use` re-export in `mod.rs` |
| `file not found for module reading` | Missing `reading.rs`, or `mod reading;` typo |

---

## Further reading

- [rust_basics.md](rust_basics.md) — structs, `impl`, and the `pub` keyword in context
- [The Rust Book — ch. 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — full treatment of the module system
