# Rust Patterns — Error Handling and Iterators

These two topics belong together because they share a foundation: both deal with values that might or might not be there. `Option` and `Result` represent "maybe a value"; iterators produce a stream of values, some of which you keep and some you discard. Once you see the connection, both become much easier to reason about.

---

## `Option<T>` — a value that might not exist

`Option` is Rust's replacement for `null`. Instead of a value silently being `null` at runtime, Rust makes the possibility explicit in the type.

```rust
Option<f32>    // either Some(f32) or None
Option<String> // either Some(String) or None
```

You cannot use an `Option` directly where a plain value is expected. The compiler forces you to handle both cases.

**Common ways to handle it:**

```rust
let maybe: Option<f32> = Some(44.0);

// unwrap with a default
let value = maybe.unwrap_or(0.0);       // 44.0, or 0.0 if None

// match — explicit handling
match maybe {
    Some(v) => println!("{}", v),
    None    => println!("no value"),
}

// if let — when you only care about Some
if let Some(v) = maybe {
    println!("{}", v);
}
```

---

## `Result<T, E>` — an operation that might fail

`Result` is how Rust represents operations that can succeed or fail. It is an enum with two variants:

```rust
Ok(value)   // success — here is the result
Err(error)  // failure — here is the error
```

Any function that can fail returns `Result`. You cannot ignore it — the compiler warns if you do.

```rust
use std::fs;

let content = fs::read_to_string("data.txt");
// content: Result<String, std::io::Error>
// you cannot use it as a String directly
```

**Common ways to handle it:**

```rust
// match — explicit
match fs::read_to_string("data.txt") {
    Ok(content) => println!("{}", content),
    Err(e)      => println!("failed: {}", e),
}

// unwrap — crash on error (prototyping only)
let content = fs::read_to_string("data.txt").unwrap();

// expect — crash with a message
let content = fs::read_to_string("data.txt")
    .expect("could not read data.txt");

// ok() — convert to Option, discard the error
let maybe_content: Option<String> = fs::read_to_string("data.txt").ok();
```

**When to use which:**
- `match` — when you want to handle or log the failure explicitly
- `unwrap` / `expect` — only during prototyping or when failure is truly impossible
- `.ok()` — when absence is acceptable and the error reason does not matter

---

## `Option` vs `Result`

| Type | Use when |
|---|---|
| `Option<T>` | The value might not exist — absence is normal, not an error |
| `Result<T, E>` | The operation might fail — failure is unexpected and needs reporting |

Sensor with no reading → `Option<f32>` (no reading is normal).
HTTP request that might fail → `Result<Response, Error>` (failure should be reported).

Converting between them:

```rust
// Result → Option (discards the error)
let opt = some_result.ok();

// Option → Result (turns None into an error)
let res = some_option.ok_or("no value");
```

---

## The `?` operator — propagate errors up

`?` is shorthand for "if this is `Err`, return it immediately; if it is `Ok`, give me the value."

```rust
fn load_data() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("data.txt")?;  // return Err if it fails
    Ok(content)
}
```

It expands to:

```rust
let content = match fs::read_to_string("data.txt") {
    Ok(v)  => v,
    Err(e) => return Err(e),
};
```

`?` only works inside a function that returns `Result`. It moves the error decision to the caller.

### Multiple error types — `Box<dyn Error>`

When several operations can fail with different error types (HTTP errors, JSON errors, IO errors), use `Box<dyn std::error::Error>` as the return type — it accepts any error:

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://example.com")?;
    let text = response.text()?;
    fs::write("out.txt", text)?;
    Ok(())
}
```

`Ok(())` returns success with no value — `()` is the unit type, meaning "nothing."

### `main` can return `Result`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ? works here
    Ok(())
}
```

When `main` returns `Err`, Rust prints the error and exits with a non-zero code.

### Recovering without crashing

```rust
// continue with None if the operation fails
let weather = fetch_weather().ok();

// use a fallback value
let city = fetch_city().unwrap_or(String::from("Unknown"));

// log the error and continue
let outdoor_temp = match fetch_weather() {
    Ok(w)  => Some(w),
    Err(e) => { eprintln!("weather unavailable: {}", e); None }
};
```

---

## Iterators — processing collections

An iterator produces values one at a time. Instead of writing index-based loops, you describe *what* to do with each element and chain the steps.

### Getting an iterator

```rust
let numbers = vec![10, 20, 30];

numbers.iter()       // yields &T  — borrows, collection usable after
numbers.into_iter()  // yields T   — consumes, collection gone after
numbers.iter_mut()   // yields &mut T — mutable, can modify elements
```

Use `.iter()` in almost all cases.

### Iterators are lazy

Nothing runs until the iterator is consumed. Building a chain is free:

```rust
let chain = numbers.iter()
    .filter(|&&x| x > 15)
    .map(|&x| x * 2);
// nothing has happened yet

for n in chain {          // now it runs
    println!("{}", n);    // 40, 60
}
```

### `.map()` — transform each element

```rust
let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
// [20, 40, 60]
```

One input → one output. Length unchanged.

### `.filter()` — keep matching elements

```rust
let big: Vec<&i32> = numbers.iter().filter(|&&x| x > 15).collect();
// [20, 30]
```

### `.filter_map()` — transform and filter in one step

The closure returns `Option`. `Some(value)` keeps it; `None` discards it. This is where iterators and `Option` meet directly:

```rust
let words = vec!["42", "abc", "7", "xyz", "100"];

let parsed: Vec<i32> = words.iter()
    .filter_map(|s| s.parse::<i32>().ok())  // parse, skip failures
    .collect();
// [42, 7, 100]
```

`.parse::<i32>()` returns `Result`. `.ok()` converts it to `Option`. `.filter_map()` keeps only `Some` values. Three connected concepts, one line.

### `.max_by()` and `.min_by()` — finding the extreme

```rust
let words = vec!["cat", "elephant", "ox"];
let longest = words.iter().max_by(|a, b| a.len().cmp(&b.len()));
// Some("elephant")
```

Returns `Option` — `None` if the iterator is empty.

**For floats, use `.partial_cmp()` instead of `.cmp()`:**

Floats have a special value `NaN` that cannot be compared to anything. Because of this, floats implement `PartialOrd` but not `Ord`. `.max_by()` needs a full comparison:

```rust
let scores: Vec<f32> = vec![72.5, 88.3, 65.0];

let highest = scores.iter().max_by(|a, b| {
    a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
});
// Some(88.3)
```

`.unwrap_or(Ordering::Equal)` handles the theoretical `NaN` case the compiler requires you to address.

### `.find()` — first matching element

```rust
let first_big = numbers.iter().find(|&&x| x > 15);
// Some(20)
```

### `.find_map()` — find the first element that produces `Some`

Like `.find()`, but the closure returns `Option` instead of `bool`. Stops at the first `Some` and returns it; skips `None`.

```rust
let sensors = vec!["Core 0", "Package id 0", "acpitz"];

let found = sensors.iter().find_map(|&name| {
    if name.starts_with("Core") { Some(name) } else { None }
});
// Some("Core 0")
```

This is useful when you want the first element that has a value — the sensor code uses it to get the first temperature reading that exists, skipping sensors that returned `None`.

### `.enumerate()` — element with its index

```rust
for (i, item) in items.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

### `.collect()` — materialize back into a collection

```rust
let result: Vec<i32> = some_iter.collect();
```

Rust needs to know the target type — provide it as a type annotation or with turbofish:

```rust
let result = some_iter.collect::<Vec<i32>>();
```

### `.for_each()` — side effects without collecting

```rust
numbers.iter().for_each(|&n| println!("{}", n));
```

### Chaining

```rust
let result: Vec<String> = data.iter()
    .filter(|s| !s.is_empty())
    .map(|s| s.to_uppercase())
    .collect();
```

Read top to bottom: take data, drop empty strings, uppercase the rest, collect.

### `.inspect()` — debugging a chain

```rust
let result: Vec<i32> = data.iter()
    .inspect(|x| eprintln!("before: {:?}", x))
    .filter(|&&x| x > 0)
    .inspect(|x| eprintln!("after: {:?}", x))
    .cloned()
    .collect();
```

Passes values through unchanged while running your closure as a side effect. Remove after debugging.

---

## Quick reference

| Method | Returns | Use for |
|---|---|---|
| `.map(f)` | iterator | Transform every element |
| `.filter(f)` | iterator | Keep matching elements |
| `.filter_map(f)` | iterator | Transform + discard `None` |
| `.find(f)` | `Option<&T>` | First matching element |
| `.max_by(f)` / `.min_by(f)` | `Option<&T>` | Largest / smallest element |
| `.enumerate()` | iterator of `(usize, &T)` | Element with its index |
| `.for_each(f)` | `()` | Side effects only |
| `.inspect(f)` | iterator | Debug peek without changing values |
| `.collect()` | collection | Materialize into `Vec`, etc. |

---

## Further reading

- [The Rust Book, Chapter 9 — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The Rust Book, Chapter 13 — Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [rust_basics.md](rust_basics.md) — structs, enums, match (Option and Result are enums)
- [rust_memory.md](rust_memory.md) — why owned types vs references matters when collecting
