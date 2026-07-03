# Build Steps

Each step tells you what to build and which docs to read first. The gap between reading and a working program is yours to close — that is the exercise.

Read the referenced docs before writing code. When you hit something unfamiliar, check the concept docs. When the crate API is unclear, check the crate docs.

---

## Step 1 — Print all CPU temperatures

**Goal:** running `cargo run` prints every sensor's label and temperature.

```
Package id 0: 56.0 °C
Core 0: 55.0 °C
Core 1: 54.0 °C
```

**Read first:** [sysinfo.md](sysinfo.md) — `Components`, `Component`, `.label()`, `.temperature()`

**Before writing any code:** explore your system from the terminal:

```bash
ls /sys/class/thermal/
cat /sys/class/thermal/thermal_zone0/temp

for f in /sys/class/hwmon/hwmon*/; do
  echo "$(basename $f): $(cat $f/name)"
done
```

The raw `temp` value is in millidegrees Celsius — divide by 1000. `44000` → `44.0 °C`.

**Your task:**
1. Add `sysinfo = "0.33"` to `Cargo.toml`
2. Import `Components` and `Component`
3. Create the components list with `new_with_refreshed_list()`
4. Iterate over it
5. Print each sensor's label and temperature — handle the `Option<f32>`

**Checklist:**
- [ ] `sysinfo` in `Cargo.toml`
- [ ] Correct imports for both `Components` and `Component`
- [ ] All sensors printed with label and temperature
- [ ] `Option<f32>` from `.temperature()` handled — no compiler errors

---

## Step 2 — Add CPU usage and outdoor temperature

**Goal:** running `cargo run` adds CPU usage and compares with outdoor temperature.

```
São Paulo: 31.2 °C outside
CPU: 68.0 °C | Usage: 47.3%
```

**Read first:**
- [sysinfo.md](sysinfo.md) — the double-refresh pattern for CPU usage
- [reqwest-serde.md](reqwest-serde.md) — HTTP requests and JSON parsing
- [rust_patterns.md](rust_patterns.md) — `Result`, `?`, and `Box<dyn Error>`

**The two APIs:**

*Location from IP (no key needed):*
URL: `http://ip-api.com/json`
Returns JSON with `city`, `lat`, `lon` and many other fields you can ignore.

*Outdoor temperature (no key needed):*
URL: `https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&current=temperature_2m`
Returns JSON with a nested `current` object containing `temperature_2m`.

**Your task:**
1. Add `reqwest` and `serde` to `Cargo.toml` with the correct features
2. Change `main` to return `Result<(), Box<dyn std::error::Error>>`
3. Add CPU usage — implement the double-refresh pattern
4. Define a struct for the location response, fetch and parse it
5. Build the Open-Meteo URL using `format!` with the coordinates
6. Define structs for the weather response (mind the nesting), fetch and parse it
7. Print the combined output

**Checklist:**
- [ ] `reqwest` and `serde` in `Cargo.toml` with correct features
- [ ] `main` returns `Result`
- [ ] CPU usage collected with double-refresh
- [ ] Location struct defined with only the fields you need
- [ ] Weather structs defined to match the nested JSON shape
- [ ] Both APIs fetched and parsed
- [ ] Combined output printed

---

## Step 3 — Organize into layers

**Goal:** split `src/main.rs` into the `domain/`, `app/`, `infra/`, and `interface/` folder structure. The program's behavior must not change.

**Read first:** [project-structure.md](project-structure.md) — the layer architecture, how Rust modules work, what goes where

**Your task:**
1. Create the four directories under `src/`
2. Move code into the right layer — one file per responsibility
3. Run `cargo build` after each file move and fix errors before continuing
4. Verify all modes produce identical output before and after

**Checklist:**
- [ ] `src/domain/` — `Reading` struct, `HealthScore` logic
- [ ] `src/infra/` — `sensors.rs`, `weather.rs`, `db.rs`
- [ ] `src/app/` — `snapshot.rs`, `watch.rs`, `history.rs`
- [ ] `src/interface/` — `cli.rs`, `display.rs`
- [ ] `src/main.rs` — module declarations and dispatch only
- [ ] `cargo build` passes with zero errors

---

## Step 4 — Add CLI subcommand modes

**Goal:** `thermal-sentinel` dispatches to different functions based on the subcommand.

```
cargo run              → default snapshot (Step 2 output)
cargo run -- peak      → "not implemented yet"
cargo run -- indicator → "not implemented yet"
cargo run -- watch     → "not implemented yet"
cargo run -- history   → "not implemented yet"
cargo run -- --help    → generated help text
```

**Read first:** [clap.md](clap.md) — `#[derive(Parser)]`, `#[derive(Subcommand)]`, `#[arg(...)]`, dispatching

**Your task:**
1. Add `clap` to `Cargo.toml`
2. Define the `Cli` struct with `#[derive(Parser)]`
3. Define the `Commands` enum with `Peak`, `Indicator`, `Watch { interval }`, `History { limit }`
4. Move existing code into `run_default()`
5. Add stub functions for each mode (print `"not implemented yet"`)
6. Wire `main` to parse and dispatch

**Checklist:**
- [ ] `clap` in `Cargo.toml` with `derive` feature
- [ ] `Cli` struct and `Commands` enum defined
- [ ] `Watch` has `--interval` / `-i` with default 30
- [ ] `History` has `--limit` / `-l` with default 10
- [ ] `cargo run` still shows the Step 2 output
- [ ] `cargo run -- --help` shows help text

---

## Step 5 — Mode: peak

**Goal:** `cargo run -- peak` prints only the hottest sensor.

```
Hottest sensor: Package id 0 → 74.0 °C
```

**Read first:** [rust_patterns.md](rust_patterns.md) — `.filter_map()` to skip sensors with no reading, `.max_by()` and `.partial_cmp()` to find the maximum float

**Your task:**
1. Implement `run_peak()`
2. Use `.filter_map()` to skip sensors where `.temperature()` returns `None`
3. Use `.max_by()` with `.partial_cmp()` to find the hottest
4. Handle the case where no sensors are found (`None` from `.max_by()`)
5. Print the label and temperature

**Checklist:**
- [ ] `.filter_map()` used to skip `None` readings
- [ ] `.max_by()` used with `.partial_cmp()` for float comparison
- [ ] Both `Some` and `None` outcomes from `.max_by()` handled
- [ ] `cargo run -- peak` prints a single line

---

## Step 6 — Mode: indicator

**Goal:** `cargo run -- indicator` prints a composite health score with a color-coded status.

```
Location:   São Paulo — 31.2 °C outside
CPU temp:   68.0 °C  (68.0% of safe limit)
CPU usage:  47.3%
Ambient Δ:  +36.8 °C above outdoor
Score:      57 / 100  ● WARM
```

**Read first:**
- [rust_basics.md](rust_basics.md) — `const`, `if` as an expression
- `colored` crate: add `colored = "2"` to `Cargo.toml`, then `use colored::Colorize;`

**The score formula:**
```
score = (clamp(cpu_temp / 100.0, 0.0, 1.0) × 0.6 + cpu_usage / 100.0 × 0.4) × 100
```

**Status thresholds:** score < 40 → COOL (green), score < 70 → WARM (yellow), else → HOT (red)

**Your task:**
1. Add `colored = "2"` to `Cargo.toml`
2. Define threshold constants
3. Collect CPU temp (hottest sensor), CPU usage, city, outdoor temp
4. Compute the score, the temp percentage, and the ambient delta
5. Map score to a color-coded label using `if` as an expression
6. Print all five lines — use `{:+.1}` for the delta to force the sign

**Checklist:**
- [ ] `colored` in `Cargo.toml`
- [ ] Threshold constants defined at module scope
- [ ] Score formula implemented
- [ ] Color-coded label assigned with `if/else` expression
- [ ] `{:+.1}` used for the ambient delta
- [ ] `cargo run -- indicator` shows all five lines

---

## Step 7 — Mode: watch

**Goal:** `cargo run -- watch` polls every 30 seconds (or `--interval N` seconds), writes each reading to a SQLite database, and prints a one-line status. Press Ctrl+C to stop.

```
Watching... press Ctrl+C to stop.
[2024-07-03T14:30:00-03:00] CPU 68.0 °C | Usage 47.3% | Outdoor 31.2 °C
[2024-07-03T14:30:30-03:00] CPU 69.1 °C | Usage 52.0% | Outdoor 31.2 °C
```

**Read first:**
- [rusqlite.md](rusqlite.md) — opening a database, creating the schema, inserting rows, `Option<T>` for nullable columns, `chrono` for timestamps
- [rust_patterns.md](rust_patterns.md) — `.ok()` to convert a weather fetch failure into `None` without crashing the loop
- [rust_basics.md](rust_basics.md) — `loop` and `thread::sleep`

**Your task:**
1. Add `rusqlite` (with `bundled`) and `chrono` to `Cargo.toml`
2. Extract your weather fetch code into a helper function returning `Result<(String, f32), ...>`
3. Open `thermal_sentinel.db` and create the schema on startup
4. Enter a `loop`: get timestamp → collect data → try weather (use `.ok()` on failure) → insert → print → sleep
5. Use `Duration::from_secs(interval)` for the sleep

**Checklist:**
- [ ] `rusqlite` and `chrono` in `Cargo.toml`
- [ ] Weather fetch extracted to a helper function
- [ ] Database opened and schema created at startup
- [ ] Loop collects and inserts readings continuously
- [ ] Weather failure stores `NULL` instead of crashing
- [ ] One-line status printed after each insert
- [ ] `cargo run -- watch -i 3` works and writes to the database

---

## Step 8 — Mode: history

**Goal:** `cargo run -- history` prints the last 10 readings (or `--limit N`) as a formatted table.

```
Timestamp                        CPU °C  Usage %  Outdoor °C  City
----------------------------------------------------------------------
2024-07-03T14:31:00-03:00         69.1     52.0        31.2  São Paulo
2024-07-03T14:30:30-03:00         68.5     49.3        31.2  São Paulo
```

**Read first:** [rusqlite.md](rusqlite.md) — `prepare`, `query_map`, `row.get`, nullable columns, table formatting

**Your task:**
1. Open `thermal_sentinel.db`
2. Prepare a `SELECT` query with `ORDER BY id DESC LIMIT ?1`
3. Map each row to a tuple with `query_map`
4. Print a header row and a divider
5. For each row, print aligned columns — use `"—"` for `NULL` fields

**Checklist:**
- [ ] Database opened (must already exist from `watch`)
- [ ] Query uses `ORDER BY id DESC` and `LIMIT ?1`
- [ ] Rows mapped with `query_map` and `row.get`
- [ ] Nullable fields shown as `"—"` when `None`
- [ ] Output is a readable aligned table
- [ ] `cargo run -- history -l 5` shows 5 rows

---

## What's next

With all eight steps complete, the project has a working CPU temperature monitor with outdoor comparison, a composite health indicator, continuous logging, and browsable history.

From here you might explore:
- Exporting history as CSV (`std::fs`, string formatting)
- Alerting when temperature exceeds a threshold (`eprintln!`, exit codes)
- Async polling with `tokio` for more precise intervals
- Packaging as a systemd service for continuous background monitoring
