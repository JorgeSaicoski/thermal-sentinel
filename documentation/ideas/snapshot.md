# Snapshot — Design

The `snapshot` command captures a full picture of the machine's thermal state at a point in time and saves it to disk. Its purpose is not to show a live reading — it is to build a historical record that can be analyzed later, by a person or by an AI.

A single snapshot tells you little. A year of snapshots tells you whether performance is degrading, whether one season is consistently worse, or whether a service made a measurable difference.

---

## What a snapshot captures

Each snapshot record contains:

- **Timestamp** — when the snapshot was taken
- **External temperature** — the ambient room temperature, entered by the user
- **Per-core data** — for each CPU core: name, frequency, usage percentage
- **Hottest core temperature** — the highest temperature across all cores, not an average (see reasoning below)
- **Health score** — a computed value derived from the delta and usage (see formula below)

---

## Why the hottest core, not the average

Thermal problems appear first in the hottest core. Averaging across all cores hides the problem — a machine with one core at 90 °C and twenty at 40 °C looks fine on average. The hottest core is the honest signal for "is it time to service this machine?"

---

## Health score formula

See [ideas/health-score.md](health-score.md) for the formula, the function signature, and where each piece lives in the architecture.

---

## External temperature — interactive prompt with caching

The user enters the external temperature interactively when running `snapshot`. The command asks once and caches the value for one hour in SQLite.

**Why not a CLI argument with a default?** A default temperature would silently corrupt the historical record every time the user forgets to pass it. The data would look valid but be wrong. An interactive prompt is harder to skip.

**Why one hour?** Room temperature does not change significantly within an hour. Asking on every snapshot would be friction without value.

**Cache behavior:**

- On each `snapshot` run, check the SQLite database for a cached temperature and its timestamp
- If the cache is less than one hour old, use it silently
- If the cache is older than one hour, or missing, ask the user
- Write the new value and timestamp to the database before proceeding

**Ctrl+C behavior:**

An interrupted run means the snapshot may be incomplete or the session was a test. The cache is not persisted if the process exits via interrupt — the next run always asks again.

To implement this: register a signal handler for `SIGINT` that clears the cached temperature from SQLite before exiting, or set a flag in the database that marks the cache as invalidated on abnormal exit.

---

## Storage

All snapshot records are saved to SQLite via `rusqlite`. See [rusqlite.md](../crates/rusqlite.md) for setup.

Suggested table structure:

```sql
CREATE TABLE snapshots (
    id            INTEGER PRIMARY KEY,
    timestamp     INTEGER NOT NULL,
    external_temp REAL NOT NULL,
    score         REAL NOT NULL
);

CREATE TABLE snapshot_cores (
    id          INTEGER PRIMARY KEY,
    snapshot_id INTEGER NOT NULL REFERENCES snapshots(id),
    name        TEXT NOT NULL,
    frequency   REAL NOT NULL,
    usage       REAL NOT NULL
);

CREATE TABLE snapshot_temps (
    id          INTEGER PRIMARY KEY,
    snapshot_id INTEGER NOT NULL REFERENCES snapshots(id),
    label       TEXT NOT NULL,
    temperature REAL NOT NULL
);

CREATE TABLE settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

`snapshot_cores` stores one row per logical CPU (`cpu0`–`cpu21`) with its frequency and usage. `snapshot_temps` stores one row per temperature sensor (`coretemp Core 0`, `coretemp Core 1`, etc.) with its label and reading. The two tables are independent — there is no join between them because `sysinfo` does not map logical CPUs to physical cores reliably.

Aggregates are computed at query time, not at save time:

```sql
-- hottest core in a snapshot
SELECT label, MAX(temperature) FROM snapshot_temps WHERE snapshot_id = ?;

-- average usage in a snapshot
SELECT AVG(usage) FROM snapshot_cores WHERE snapshot_id = ?;

-- max temperature per day
SELECT DATE(timestamp, 'unixepoch'), MAX(temperature)
FROM snapshot_temps
JOIN snapshots ON snapshots.id = snapshot_temps.snapshot_id
GROUP BY DATE(timestamp, 'unixepoch');
```

The score saved in `snapshots` is computed at write time using the hottest sensor temperature and average CPU usage — but the raw data is always there to recompute with a different formula later.

`settings` holds the cached external temperature (`key = 'external_temp'`) and its timestamp (`key = 'external_temp_ts'`).

---

## Where it fits in the architecture

- `domain/snapshot_record.rs` — the struct that holds one complete snapshot
- `domain/score.rs` — the health score formula (pure domain logic, no external crates)
- `infra/sensors.rs` — provides per-core data via `SensorReader`
- `infra/db.rs` — writes the snapshot record to SQLite
- `app/snapshot.rs` — assembles the record: reads sensors, asks for external temp if needed, computes score, calls `db::save`
- `interface/display.rs` — confirms to the user that the snapshot was saved

The score formula belongs in `domain/` because it is pure computation — no hardware, no IO, no external crates. It takes numbers in and returns a number out.

---

## Further reading

- [ideas/health-score.md](health-score.md) — alternative scoring approaches considered before this formula was chosen
- [crates/rusqlite.md](../crates/rusqlite.md) — SQLite setup and usage
- [architecture/minimal-startup.md](../architecture/minimal-startup.md) — how the layers connect
