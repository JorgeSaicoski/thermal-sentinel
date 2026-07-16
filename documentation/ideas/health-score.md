# Health Score

The health score answers a single question: *is this CPU running hotter than it should for the work it is doing?*

It is a pure computation — no hardware reads, no IO, no external crates. It takes numbers in and returns a number out. That makes it domain logic: it lives in `domain/score.rs`.

---

## The formula

Two components:

**Temperature delta** — the gap between the hottest sensor reading and the external temperature:

```
delta = hottest_temp - external_temp
```

A large delta on a cool day means the CPU is generating heat it cannot dissipate. That points to dried thermal paste, a clogged heatsink, or poor airflow.

**Usage weight** — a logarithmic curve that makes low usage more sensitive than high usage:

```
weight = log(usage + 1) / log(101)
```

At 0% usage the weight approaches zero. At heavy load, high temperatures are expected — the weight reflects that by growing slowly at the high end.

**Final score:**

```
score = delta * weight
```

A high score while usage is low is the red flag: heat without cause. The score is only meaningful as a trend across multiple readings, not in isolation.

---

## Where the inputs come from

The score function needs three pieces of data:

- **Hottest sensor temperature** — found by a helper method on `CpuInfo`; the sensor API returns individual readings, not a max
- **Average CPU usage** — `sysinfo` provides `global_cpu_usage()` directly as a single `f32`; no averaging needed
- **External temperature** — entered by the user interactively; cached in `main.rs` alongside the loop

None of these come from inside `domain/`. The score function receives them already computed — it does not fetch, prompt, or search.

---

## Finding the hottest temperature

`sysinfo` gives you individual sensor readings. Finding the hottest is a domain operation — it belongs on `CpuInfo` as a method:

```rust
// domain/cpu_info.rs

impl CpuInfo {
    pub fn hottest(temps: &[CpuInfo]) -> Option<f32> {
        temps.iter().map(|c| c.temperature).reduce(f32::max)
    }
}
```

`reduce(f32::max)` walks the iterator and keeps the largest value. It returns `Option<f32>` because if the slice is empty there is no maximum to return.

This is an **associated function** — it takes a slice rather than `&self`, because it operates on a collection of `CpuInfo`, not a single one. You call it as `CpuInfo::hottest(&temps)`.

---

## The function signature

```rust
// domain/score.rs

pub fn compute(cpu_temp: f32, avg_usage: f32, external_temp: f32) -> f32 {
    // apply formula
}
```

Three plain numbers in, one number out. The score function does not know how those numbers were obtained — that is the app layer's job.

---

## The external temperature — main.rs concern

The external temperature is not a CLI argument. The user enters it interactively, and it is cached so they are not asked on every loop iteration.

This caching logic lives in `main.rs` alongside the loop — consistent with how other commands handle their loops. The score function does not know where the temperature came from.

A counter approach works well: track how many iterations have passed, and when `interval * count >= 3600`, ask the user again and reset the counter.

```rust
// conceptual pattern in main.rs

let mut external_temp: f32 = ask_user_for_temp();
let mut count: u64 = 0;

loop {
    if interval * count >= 3600 {
        external_temp = ask_user_for_temp();
        count = 0;
    }

    let hottest = CpuInfo::hottest(&temps).unwrap_or(0.0);
    let usage   = reader.global_usage();
    let score   = domain::score::compute(hottest, usage, external_temp);
    // display score
    count += 1;
    std::thread::sleep(Duration::from_secs(interval));
}
```

`ask_user_for_temp()` belongs in `interface/` — it reads from stdin and returns an `f32`. `main.rs` calls it; the domain layer never sees it.

---

## Where each piece lives

| Piece | Location |
|---|---|
| Formula | `domain/score.rs` |
| `hottest()` helper | `domain/cpu_info.rs` |
| Loop, counter, cached temperature | `main.rs` |
| Interactive temperature prompt | `interface/` |
| Score display | `interface/display.rs` |

---

## Further reading

- [architecture/project-structure.md](../architecture/project-structure.md) — how domain, app, and interface relate
- [ideas/snapshot.md](snapshot.md) — how a snapshot record is assembled and saved
- [rust/rust_patterns.md](../rust/rust_patterns.md) — slices and borrows
