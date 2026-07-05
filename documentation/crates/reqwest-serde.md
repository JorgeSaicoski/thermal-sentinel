# Using `reqwest` and `serde` — HTTP Requests and JSON

These two crates are almost always used together: `reqwest` makes the HTTP request and gives you the response body; `serde` parses that body from JSON into a Rust struct. This document covers both.

## Add to `Cargo.toml`

```toml
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde   = { version = "1",    features = ["derive"] }
```

Feature flags matter:
- `blocking` — gives you a synchronous API. Without it, `reqwest` is async-only, which requires `async`/`await`. The blocking API is simpler to start with.
- `json` (reqwest) — adds the `.json::<T>()` method to parse a response body directly into a Rust type.
- `derive` (serde) — lets you add `#[derive(Deserialize)]` to a struct instead of writing parsing code manually.

After adding, run `cargo build` once to download and compile both crates.

---

## Making a GET request

```rust
let response = reqwest::blocking::get("https://example.com/data")?;
```

`?` propagates the error if the request fails (no network, bad URL, server error). The calling function must return `Result` — see [rust_patterns.md](rust_patterns.md).

The response is not the data yet — it is the HTTP response object. To get the body as text:

```rust
let body = response.text()?;
```

To get the body parsed directly as JSON into a Rust type:

```rust
let data = response.json::<MyStruct>()?;
```

Or chained in one line:

```rust
let data = reqwest::blocking::get("https://example.com/data")?.json::<MyStruct>()?;
```

---

## Defining structs for JSON with `serde`

Add `#[derive(Deserialize)]` to any struct and serde will parse JSON into it automatically. The struct field names must match the JSON keys exactly (case-sensitive).

If the JSON response is:

```json
{ "name": "São Paulo", "population": 12300000 }
```

Define:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct City {
    name: String,
    population: u64,
}
```

Fields not in your struct are ignored — you only declare what you need.

### Nested JSON → nested structs

When the JSON has a nested object, use a nested struct:

```json
{
  "current": {
    "temperature": 31.2,
    "humidity": 65
  }
}
```

```rust
#[derive(Deserialize)]
struct WeatherResponse {
    current: CurrentConditions,
}

#[derive(Deserialize)]
struct CurrentConditions {
    temperature: f32,
    humidity: u32,
}
```

The nesting in the struct must mirror the nesting in the JSON.

### Field name mismatches

If the JSON key uses a different naming convention than Rust (e.g., `temperatureC` in JSON, `temperature_c` in Rust), use `#[serde(rename)]`:

```rust
#[derive(Deserialize)]
struct Conditions {
    #[serde(rename = "temperatureC")]
    temperature_c: f32,
}
```

---

## Building URLs dynamically

When you need to put variables into a URL, use `format!`:

```rust
let lat = -23.5505_f64;
let lon = -46.6333_f64;

let url = format!(
    "https://api.example.com/weather?lat={}&lon={}",
    lat, lon
);
```

---

## HTTP vs HTTPS

`reqwest` supports both. Be aware that some free APIs (like ip-api.com's free tier) only expose plain HTTP endpoints. Make sure the URL starts with `http://` in those cases — `reqwest` will follow the correct protocol automatically based on the URL scheme.

---

## The full pattern

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct LocationResponse {
    // declare only the fields you need
}

fn fetch_location() -> Result<LocationResponse, Box<dyn std::error::Error>> {
    let data = reqwest::blocking::get("http://ip-api.com/json")?
        .json::<LocationResponse>()?;
    Ok(data)
}
```

This is the shape every fetch function takes: one return type, one or two `?` operations, `Ok(data)` at the end.

---

## Further reading

- [reqwest blocking docs](https://docs.rs/reqwest/latest/reqwest/blocking/index.html)
- [serde docs](https://serde.rs)
- [rust_patterns.md](../rust/rust_patterns.md) — `Result`, `?`, and `Box<dyn Error>`
- [rust_basics.md](../rust/rust_basics.md) — structs and `#[derive]`
- [ideas/data-sources.md](../ideas/data-sources.md) — APIs and data sources you could fetch
