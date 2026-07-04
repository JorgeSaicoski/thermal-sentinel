# Using `rusqlite` and `chrono` — SQLite Database and Timestamps

SQLite is an embedded database — a single file on disk, no server, no configuration. `rusqlite` is the standard Rust binding for it. `chrono` is Rust's date and time library. In this project they always appear together: `chrono` generates the timestamp; `rusqlite` stores it.

## Add to `Cargo.toml`

```toml
rusqlite = { version = "0.32", features = ["bundled"] }
chrono   = "0.4"
```

`bundled` compiles SQLite directly into your binary — nothing needs to be installed on the target machine.

---

## Why SQLite over a plain file

You could append readings to a CSV file. SQLite gives you:

- **Queries** — find averages, peaks, readings above a threshold — with SQL
- **Transactions** — if the program crashes mid-write, the database is not corrupted
- **No parsing** — you do not write CSV parsing code
- **Free tooling** — `sqlite3` in your terminal, or [DB Browser for SQLite](https://sqlitebrowser.org), can open the file directly

---

## SQL primer — the four statements you need

```sql
-- create a table (if it does not already exist)
CREATE TABLE IF NOT EXISTS readings (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT    NOT NULL,
    cpu_temp  REAL    NOT NULL
);

-- insert a row
INSERT INTO readings (timestamp, cpu_temp) VALUES (?1, ?2);

-- query rows
SELECT timestamp, cpu_temp FROM readings ORDER BY id DESC LIMIT 10;

-- run only in emergencies
DROP TABLE readings;
```

SQLite types:
- `INTEGER` — whole numbers
- `REAL` — floating-point numbers (`f64` in Rust)
- `TEXT` — strings
- `NULL` — absence of a value (nullable columns omit `NOT NULL`)

---

## Opening (or creating) a database

```rust
use rusqlite::Connection;

let conn = Connection::open("data.db")?;
```

If `data.db` does not exist, SQLite creates it. If it does, SQLite opens it. The `?` propagates any IO error.

---

## Creating the schema

Run this once at startup — `IF NOT EXISTS` makes it safe to run every time:

```rust
conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS readings (
        id           INTEGER PRIMARY KEY AUTOINCREMENT,
        timestamp    TEXT    NOT NULL,
        cpu_temp     REAL    NOT NULL,
        cpu_usage    REAL    NOT NULL,
        outdoor_temp REAL,
        city         TEXT
    );"
)?;
```

`execute_batch` runs one or more SQL statements that do not return rows (CREATE, INSERT, UPDATE, DROP).

Column notes:
- `outdoor_temp` and `city` are nullable (no `NOT NULL`) — a network failure should not prevent saving a reading
- `AUTOINCREMENT` on `id` means you never set it manually

---

## Generating a timestamp with `chrono`

```rust
use chrono::Local;

let timestamp = Local::now().to_rfc3339();
// e.g. "2024-07-03T14:30:00-03:00"
```

`Local::now()` returns the current time in the system's local timezone. `.to_rfc3339()` formats it as a standard string that sorts correctly in lexicographic order (important for `ORDER BY timestamp`).

---

## Inserting a row

```rust
use rusqlite::params;

conn.execute(
    "INSERT INTO readings (timestamp, cpu_temp, cpu_usage, outdoor_temp, city)
     VALUES (?1, ?2, ?3, ?4, ?5)",
    params![timestamp, cpu_temp, cpu_usage, outdoor_temp, city],
)?;
```

`?1`, `?2`, etc. are positional placeholders — `rusqlite` substitutes the `params![]` values in order. This is not string interpolation: `rusqlite` handles escaping, so there is no risk of SQL injection.

`params![]` accepts any type `rusqlite` knows how to convert: `f32`, `f64`, `String`, `&str`, `i32`, `Option<T>`, etc.

### Nullable columns with `Option`

Pass `Option<T>` for nullable columns. `None` writes `NULL`; `Some(v)` writes the value:

```rust
let outdoor_temp: Option<f32> = Some(31.2);   // writes 31.2
let city: Option<String> = None;               // writes NULL
```

---

## Querying rows

Reading rows back takes two steps: **prepare** the SQL, then **map** each row to a Rust type.

### Prepare

```rust
let mut stmt = conn.prepare(
    "SELECT timestamp, cpu_temp, cpu_usage, outdoor_temp, city
     FROM readings
     ORDER BY id DESC
     LIMIT ?1"
)?;
```

Preparing compiles the SQL once. Reuse `stmt` if you run the same query multiple times.

### Map rows to Rust types

```rust
let rows = stmt.query_map([limit as i64], |row| {
    Ok((
        row.get::<_, String>(0)?,           // column 0 → String
        row.get::<_, f64>(1)?,              // column 1 → f64
        row.get::<_, f64>(2)?,              // column 2 → f64
        row.get::<_, Option<f64>>(3)?,      // column 3 → nullable f64
        row.get::<_, Option<String>>(4)?,   // column 4 → nullable String
    ))
})?;
```

- `row.get::<_, T>(index)` reads column `index` (0-based) and converts to type `T`
- The `_` lets Rust infer the first type parameter automatically
- For nullable columns, use `Option<T>` — `rusqlite` maps SQL `NULL` to `None`

`.query_map()` returns an iterator of `Result<YourTuple>` — each row can fail independently (disk error, type mismatch). Use `?` inside the loop:

```rust
for row in rows {
    let (timestamp, cpu_temp, cpu_usage, outdoor_temp, city) = row?;
    // use the values
}
```

---

## Formatting table output

To print data aligned in columns, use width specifiers in the format string:

```rust
println!("{:<30} {:>8} {:>8} {:>10}",
    "Timestamp", "CPU °C", "Usage %", "Outdoor");
println!("{}", "-".repeat(60));
```

- `{:<30}` — left-align, minimum 30 characters wide
- `{:>8}` — right-align, minimum 8 characters wide
- `{:>8.1}` — right-align, 8 wide, 1 decimal place

For `Option` fields, convert to a display string:

```rust
let outdoor_str = outdoor_temp
    .map(|t| format!("{:.1}", t))
    .unwrap_or(String::from("—"));
```

`Some(31.2)` → `"31.2"`, `None` → `"—"`.

---

## Inspecting the database file directly

The `.db` file lives in the directory where you ran `cargo run`. Open it with:

```bash
sqlite3 thermal_sentinel.db
sqlite> SELECT * FROM readings ORDER BY id DESC LIMIT 5;
sqlite> .quit
```

Or use [DB Browser for SQLite](https://sqlitebrowser.org) for a visual interface.

---

## Further reading

- [rusqlite docs](https://docs.rs/rusqlite/latest/rusqlite/)
- [chrono docs](https://docs.rs/chrono/latest/chrono/)
- [SQLite docs](https://www.sqlite.org/docs.html)
- [rust_patterns.md](rust_patterns.md) — `Result`, `?`, and `.ok()` for graceful error handling inside loops
- [rust_basics.md](rust_basics.md) — `loop`, `thread::sleep` for the polling pattern
