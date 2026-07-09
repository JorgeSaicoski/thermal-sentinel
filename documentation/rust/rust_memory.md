# Rust Memory — Ownership, Borrowing, and the Stack vs Heap

This is the most important concept unique to Rust. Most bugs that other languages ship at runtime — memory leaks, use-after-free, data races — Rust catches at compile time, using the rules described here.

---

## Why memory management matters

Every value your program creates has to live *somewhere* in memory. The question is: who decides when to clean it up?

| Language | Strategy | Cost |
|---|---|---|
| C / C++ | You manage manually (`malloc` / `free`) | Easy to forget — crashes, security bugs |
| Python / Go / Java | Garbage collector runs in the background | Safe, but adds runtime overhead and pauses |
| Rust | Compiler enforces rules at build time | No runtime cost, no GC, no crashes |

---

## The Stack and the Heap

### Stack — fast, automatic, fixed size

Every function call pushes a **stack frame** — a block of memory holding the function's local variables. When the function returns, the frame is popped and everything in it is instantly gone.

Types that live on the stack must have a known, fixed size at compile time:

```
i32, f32, bool, char     — scalar types
(i32, f32)               — tuples of fixed-size types
[f32; 4]                 — arrays with a known length
```

### Heap — flexible, manual, slower

The heap is a large pool for values whose size is unknown at compile time or that need to outlive a function call. You request a block; the OS gives you a pointer to it.

Types that live on the heap:

```
String, Vec<T>, Box<T>, HashMap
```

When you create a `String`, Rust puts two things in memory:

```
Stack:                   Heap:
┌─────────────┐         ┌──────────────────┐
│ pointer ────┼────────►│ h e l l o        │
│ length: 5   │         └──────────────────┘
│ capacity: 5 │
└─────────────┘
```

The small fixed-size header (pointer + metadata) lives on the stack. The actual data lives on the heap.

---

## Ownership — one owner, automatic cleanup

Rust's rule:

> Every value has exactly one owner. When the owner goes out of scope, the value is dropped and its memory is freed.

```rust
fn main() {
    let label = String::from("Core 0");  // label owns this String
    println!("{}", label);
}  // label goes out of scope — String is freed here, automatically
```

No `free()`. No GC. The compiler inserts cleanup at the right place.

### Move — transferring ownership

Assigning a heap value to another variable **moves** ownership. The original variable is invalidated.

```rust
let a = String::from("Core 0");
let b = a;             // ownership moves to b

println!("{}", b);     // ok
println!("{}", a);     // ERROR: a is no longer valid
```

```
error[E0382]: borrow of moved value: `a`
```

### Copy — stack types are duplicated, not moved

Scalar types (`i32`, `f32`, `bool`, etc.) implement `Copy`. Assignment duplicates the value instead of moving it — the original stays valid.

```rust
let x: i32 = 5;
let y = x;             // x is copied, not moved
println!("{}", x);     // still valid
```

---

## Borrowing — reading without owning

A **borrow** lets a function read or modify a value without taking ownership. When the borrow ends, the original owner is unaffected.

```rust
fn print_label(label: &String) {   // borrows label, does not own it
    println!("{}", label);
}

fn main() {
    let sensor = String::from("Core 0");
    print_label(&sensor);           // lend it
    println!("{}", sensor);         // sensor is still ours
}
```

`&` creates a reference. `&T` is read-only; `&mut T` is writable.

### The borrow rules

Rust enforces these at compile time:

1. **You can have many shared borrows (`&T`) at once** — read-only, no conflict
2. **You can have exactly one mutable borrow (`&mut T`) at a time** — no shared borrows simultaneously

```rust
fn double(temp: &mut f32) {
    *temp *= 2.0;    // * dereferences the pointer
}

let mut reading = 44.0_f32;
double(&mut reading);
println!("{}", reading);   // 88.0
```

Violating the rules:

```
error[E0502]: cannot borrow `reading` as mutable because it is also borrowed as immutable
```

### A borrow cannot outlive the value it points to

```rust
fn main() {
    let reference;
    {
        let label = String::from("Core 0");
        reference = &label;
    }  // label is dropped here
    println!("{}", reference);   // ERROR — reference now points to freed memory
}
```

```
error[E0597]: `label` does not live long enough
```

In C, this compiles and produces garbage or a crash. In Rust, it is a compile-time error.

### You cannot return a reference to a local variable

```rust
fn get_label() -> &str {       // ERROR
    let label = String::from("Core 0");
    &label                     // label is dropped when the function returns
}
```

Return an owned value instead:

```rust
fn get_label() -> String {     // correct
    String::from("Core 0")     // ownership moves to the caller
}
```

---

## Lifetimes

Every reference has a **lifetime** — the span of code where it is valid. The compiler infers them automatically in most cases. You only write lifetime annotations when a function returns a reference and the compiler cannot figure out which input it comes from:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` says: "the returned reference lives as long as both inputs do." You will encounter this when writing functions that return references — the compiler will ask for annotations when it needs them.

---

## `clone()` — explicit deep copy

When you genuinely need two independent copies of a heap value:

```rust
let a = String::from("Core 0");
let b = a.clone();   // explicit copy of the heap data
println!("{} {}", a, b);   // both valid
```

`clone()` is intentionally explicit — you are saying "I know this copies data, and I mean it." Do not reach for `.clone()` every time you get a borrow error; often the correct fix is to borrow instead.

---

## `&str` vs `String` — borrowed vs owned strings

Rust has two string types and they are not interchangeable.

| Type | What it is | Lives on |
|---|---|---|
| `String` | An owned, growable string | Heap |
| `&str` | A borrowed view into a string | Anywhere |

**`String`** — you own it. You can modify it, store it in a struct, and pass it to other functions.

**`&str`** — you are borrowing a view of someone else's string. You cannot store it in a struct that outlives the borrow.

Many library methods return `&str` — a temporary view of internal data. If you need to keep that value, you must convert it to an owned `String`:

```rust
// .label() returns &str — a borrowed view
let borrowed: &str = component.label();

// to store it, convert to owned String
let owned: String = component.label().to_string();
```

Two equivalent ways to convert:

```rust
let s = some_str.to_string();        // method on &str
let s = String::from(some_str);      // associated function on String
```

The compiler tells you when you have this wrong:

```
error[E0308]: mismatched types
  expected `String`, found `&str`
help: try using a conversion method: `label.to_string()`
```

When you see `expected String, found &str` — add `.to_string()` at the point where you use the value.

---

## Good practices

**Prefer `&str` over `&String` in function parameters** — `&str` accepts string literals, `String` references, and anything string-like. `&String` only accepts `String`.

**Prefer `&[T]` over `&Vec<T>` in function parameters** — same reason: more general.

**Use `mut` only where necessary** — immutability by default makes code clearer and prevents accidental modification.

**Borrow before cloning** — ask "do I need a separate copy, or just to read this?" before calling `.clone()`.

**`Rc<T>` / `Arc<T>` for shared ownership** — when you genuinely need multiple owners, use `Rc<T>` (single-threaded) or `Arc<T>` (multi-threaded). Both use reference counting: the value is freed when the last owner is gone. Do not use these to work around borrow errors — they are for architecturally justified shared ownership.

---

## Summary

```
┌──────────────────────────────────────────────────────────────────┐
│                        RUST MEMORY MODEL                         │
├──────────────┬───────────────────────────────────────────────────┤
│ STACK        │ Fixed-size values, freed when the function returns │
├──────────────┼───────────────────────────────────────────────────┤
│ HEAP         │ Flexible values, freed when the owner is dropped   │
├──────────────┼───────────────────────────────────────────────────┤
│ OWNERSHIP    │ One owner. Owner dropped → value freed.            │
│ BORROWING    │ &T = read-only. &mut T = one writer, no readers.  │
│ NO GC        │ All enforced at compile time. Zero runtime cost.   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Further reading

- [The Rust Book, Chapter 4 — Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [rust_basics.md](rust_basics.md) — variables, types, structs, enums
- [rust_patterns.md](rust_patterns.md) — Result and iterators
