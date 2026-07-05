# Health Score — Ideas

The health score is an optional extension: a single number or label that tells you at a glance whether your CPU is running well. This document offers several approaches — not to prescribe one, but to give you a starting point and something to think through.

There is no single right answer. The interesting part is picking an approach and justifying it.

---

## What it represents

A health score takes multiple signals (CPU temperature, CPU usage, outdoor temperature) and collapses them into one judgment. The challenge: those signals have different units and different meanings. The design question is how to combine them in a way that is useful and honest.

---

## Approach 1 — Simple temperature threshold

The simplest version: assign a status based on CPU temperature alone.

| Temperature | Status |
|---|---|
| Below 70 °C | Cool |
| 70 – 85 °C | Warm |
| Above 85 °C | Hot |

**Pros:** Trivial to implement. Easy to explain.
**Cons:** Ignores CPU usage. A 65 °C CPU running at 100% for an hour is not "cool."

---

## Approach 2 — Combined temperature and usage

Score temperature and usage separately, then average them:

```
score = (temp_score + usage_score) / 2
```

Where each component is normalized to 0–100:

```
temp_score  = 100 − (cpu_temp / 100.0 × 100)
usage_score = 100 − cpu_usage
```

**Pros:** Uses both available signals. Gives a numeric value that can trend over time.
**Cons:** The 100 °C ceiling is arbitrary — meaningful for some CPUs, not others.

---

## Approach 3 — Delta from ambient temperature

The gap between CPU temperature and outdoor temperature may be more meaningful than the absolute CPU temperature. A 60 °C CPU on a 40 °C day is different from a 60 °C CPU on a 10 °C day.

```
delta = cpu_temp − outdoor_temp
```

| Delta | Status |
|---|---|
| Below 40 °C | Cool |
| 40 – 60 °C | Warm |
| Above 60 °C | Hot |

**Pros:** Contextual — accounts for ambient conditions. This is what the outdoor temperature feature enables.
**Cons:** Requires weather data. If the weather fetch fails, there is no delta to compute.
**Question worth answering:** What should the fallback be when outdoor temperature is unavailable — Approach 1, or no score at all?

---

## Approach 4 — Weighted composite

Assign explicit weights to each signal:

```
score = (temp_normalized × 0.6) + (usage_normalized × 0.4)
```

**Pros:** Priorities are explicit and easy to tune.
**Cons:** The weights are subjective. "I chose 0.6 because it felt right" is honest but weak without a rationale.

---

## Where it fits in the code

The score is derived from a `Reading` — it does not come from hardware or the network. That makes it pure domain logic. It belongs in `domain/score.rs`, not in `infra/`.

```rust
pub enum Status {
    Cool,
    Warm,
    Hot,
}

pub struct HealthScore {
    pub value: f32,
    pub status: Status,
    pub ambient_delta: Option<f32>,
}

impl HealthScore {
    pub fn from_reading(reading: &Reading) -> HealthScore {
        // your formula here
    }
}
```

`from_reading` takes `&Reading` — a borrow, not ownership — because it only reads the data.

Where to call it:

- `domain/score.rs` — the formula and types (no external crates, no IO)
- `app/snapshot.rs` — call `HealthScore::from_reading(&reading)` after assembling the `Reading`
- `interface/display.rs` — format and print the result

The dependency rule holds: `score.rs` imports from `domain/reading.rs` only — nothing from `infra` or `app`.

---

## Further reading

- [architecture/project-structure.md](../architecture/project-structure.md) — how domain, app, and interface relate
- [rust/rust_basics.md](../rust/rust_basics.md) — `enum`, `impl`, and `match` for the Status type
- [rust/rust_patterns.md](../rust/rust_patterns.md) — `Option` for `ambient_delta`
