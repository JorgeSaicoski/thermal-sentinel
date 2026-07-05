# Data Sources — What You Could Fetch with reqwest

`reqwest` and `serde` fetch data from any URL and parse the JSON response. The pattern is always the same — what changes is what you ask for and what you do with the answer.

This document lists some data sources and what questions they can answer. None of these are required. Pick something that makes your project more interesting to you.

---

## The pattern (review)

Every external data source follows this shape:

```rust
#[derive(Deserialize)]
struct SomeResponse {
    // only the fields you need
}

fn fetch() -> Result<SomeResponse, Box<dyn std::error::Error>> {
    let data = reqwest::blocking::get("https://some-api.com/endpoint")?
        .json::<SomeResponse>()?;
    Ok(data)
}
```

Whatever the source, the adapter lives in `infra/` and returns a domain type, not raw JSON.

---

## Weather and temperature

**Open-Meteo** — free, no API key required

```
https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&current=temperature_2m
```

Returns current outdoor temperature in °C. You need coordinates — pair it with an IP geolocation call to get them automatically, or hardcode your city.

**wttr.in** — free, no API key

```
https://wttr.in/?format=j1
```

Returns weather for your location (detected by IP). Simpler to start with since you don't need to pass coordinates.

What you can ask: How does CPU temperature relate to outdoor temperature? Does the machine run hotter on warmer days?

---

## Location and time zone

**ip-api.com** — free tier, no API key, plain HTTP only

```
http://ip-api.com/json
```

Returns city, country, latitude, longitude, and timezone based on your public IP. Useful for getting coordinates to pass to a weather API.

What you can ask: Where is this machine? Which timezone should timestamps use?

---

## Air quality

**Open-Meteo air quality** — free, no API key

```
https://air-quality-api.open-meteo.com/v1/air-quality?latitude=LAT&longitude=LON&current=pm2_5,european_aqi
```

Returns particulate matter (PM2.5) and an air quality index.

What you can ask: Is poor air quality correlated with anything you're measuring locally?

---

## Time

If you only want to compare data across time of day or day of week — no API needed. The standard library gives you the current time:

```rust
use std::time::{SystemTime, UNIX_EPOCH};

let epoch_secs = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();
```

Add `chrono` to `Cargo.toml` first (`chrono = "0.4"`), then use it for human-readable formats and day-of-week access:

```rust
use chrono::Local;

let now = Local::now();
let hour = now.hour();          // 0–23
let weekday = now.weekday();    // Mon, Tue, ...
let timestamp = now.to_rfc3339();
```

What you can ask: Does CPU temperature spike at a specific time of day? Does usage differ between weekdays and weekends?

---

## Other public APIs

Any endpoint that returns JSON works with the same `reqwest` + `serde` pattern. A few more examples:

| Source | What it gives | Notes |
|---|---|---|
| `https://api.coindesk.com/v1/bpi/currentprice.json` | Bitcoin price | Silly but good for practicing JSON parsing |
| `https://api.sunrise-sunset.org/json?lat=LAT&lng=LON` | Sunrise/sunset times | Does CPU temperature differ between day and night? |
| `https://api.open-meteo.com/v1/forecast?...&hourly=temperature_2m` | Hourly forecast | Compare current temp with what was expected |

---

## Things to consider

**No-auth vs API key APIs.** All examples above require no authentication. If you want to use an API that requires a key, keep the key out of source code — read it from an environment variable:

```rust
let key = std::env::var("MY_API_KEY")?;
```

**HTTP vs HTTPS.** Some free API tiers (like ip-api.com's free endpoint) use plain HTTP. Make sure your URL starts with `http://` in those cases.

**Failure is normal.** Network calls fail. Design your adapter to return `Result` and let `snapshot.rs` decide what to do when the call fails — usually `outdoor_temp: None` rather than crashing.

---

## Further reading

- [crates/reqwest-serde.md](../crates/reqwest-serde.md) — the full request and parsing pattern
- [rust/rust_patterns.md](../rust/rust_patterns.md) — `Result`, `?`, and `.ok()` for graceful failure
- [architecture/minimal-startup.md](../architecture/minimal-startup.md) — where adapters live and what they return
