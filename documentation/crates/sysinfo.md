# Using `sysinfo` — CPU Temperature and Usage

`sysinfo` is a cross-platform crate for reading hardware and OS information. In this project you use it for two things: CPU temperature from hardware sensors, and CPU usage as a percentage.

## Add to `Cargo.toml`

```toml
[dependencies]
sysinfo = "0.33"
```

---

## Reading CPU temperature

### The types you need

| Type | What it is |
|---|---|
| `Components` | A collection of all hardware sensors on the machine |
| `Component` | A single sensor — import this to unlock its methods |
| `Components::new_with_refreshed_list()` | Creates the collection and populates it immediately |
| `.label()` | The sensor name as `&str` — e.g. `"Core 0"`, `"Package id 0"` |
| `.temperature()` | Current temperature as `Option<f32>` in degrees Celsius |

### Importing

```rust
use sysinfo::{Components, Component};
```

Both must be in scope. `Components` creates the collection; `Component` unlocks the `.label()` and `.temperature()` methods. Import the trait, and its methods become available.

### Reading all sensors

```rust
use sysinfo::{Components, Component};

let components = Components::new_with_refreshed_list();

for component in &components {
    if let Some(temp) = component.temperature() {
        println!("{}: {:.1} °C", component.label(), temp);
    }
}
```

`component.temperature()` returns `Option<f32>` — some sensors may not have a reading. The `if let Some(temp)` pattern skips those automatically.

### Why `Option<f32>` and not just `f32`?

Not every sensor is guaranteed to report a value. A sensor might be temporarily unavailable, or the driver might not expose it. Rust forces you to acknowledge this possibility — you cannot accidentally treat a missing reading as zero.

---

## Reading CPU usage

CPU usage is a **rate**, not a stored value. The OS calculates it by measuring how many clock cycles were spent doing work versus idle over a time window. To get a meaningful number, you need to take two measurements with a gap between them.

### The types you need

| Type | What it is |
|---|---|
| `System` | The main struct for system information |
| `System::new()` | Creates an empty `System` — nothing populated yet |
| `.refresh_cpu_usage()` | Takes one CPU snapshot — must be called twice |
| `.global_cpu_usage()` | Returns overall CPU usage as `f32` (0.0–100.0) |
| `sysinfo::MINIMUM_CPU_UPDATE_INTERVAL` | The minimum pause needed between snapshots |

### The double-refresh pattern

```rust
use sysinfo::System;
use std::thread;

let mut sys = System::new();

sys.refresh_cpu_usage();                              // first snapshot
thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL); // wait
sys.refresh_cpu_usage();                              // second snapshot

let usage = sys.global_cpu_usage();
println!("CPU usage: {:.1}%", usage);
```

`sysinfo::MINIMUM_CPU_UPDATE_INTERVAL` is a constant defined by the crate — the shortest interval that produces a reliable reading (typically ~200ms). Use it rather than hardcoding a number.

`sys` must be `mut` because `.refresh_cpu_usage()` modifies the struct's internal state. Rust requires explicit `mut` for any variable that will be changed.

### Using both temperature and usage together

`Components` and `System` are separate objects — they can coexist:

```rust
use sysinfo::{Components, Component, System};
use std::thread;

let components = Components::new_with_refreshed_list();

let mut sys = System::new();
sys.refresh_cpu_usage();
thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
sys.refresh_cpu_usage();

let usage = sys.global_cpu_usage();

for component in &components {
    if let Some(temp) = component.temperature() {
        println!("{}: {:.1} °C | Usage: {:.1}%", component.label(), temp, usage);
    }
}
```

---

## Reusing state across multiple readings

When you read sensors once and stop, creating `Components` or `System` on each call is fine. But when you read in a loop — every 30 seconds, every minute — recreating those objects every tick is wasteful. Each creation reconnects to the OS, allocates memory, and does a full scan from scratch.

The right pattern: **create once, refresh each tick.**

`sysinfo` supports this directly. Every object that can be created can also be refreshed:

```rust
// create once — expensive, do this outside the loop
let mut components = Components::new_with_refreshed_list();

// each tick — cheap, just updates the values
components.refresh(true);

// now read from components as normal
for component in &components {
    if let Some(temp) = component.temperature() {
        println!("{}: {:.1} °C", component.label(), temp);
    }
}
```

`refresh(true)` updates each sensor's value in place. The `true` argument tells `sysinfo` to also check for new sensors that may have appeared — pass `false` if you only want to update known sensors.

The same applies to `System`:

```rust
// create once
let mut sys = System::new_with_specifics(
    RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
);

// each tick
std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
sys.refresh_cpu_all();

// read as normal
for cpu in sys.cpus() {
    println!("{}: {:.1}%", cpu.name(), cpu.cpu_usage());
}
```

### Wrapping in a struct

When you need to carry this state through your application, put it in a struct with a `new()` constructor. This keeps initialization separate from reading:

```rust
struct SensorReader {
    components: Components,
}

impl SensorReader {
    fn new() -> SensorReader {
        SensorReader {
            components: Components::new_with_refreshed_list(),
        }
    }

    fn read(&mut self) -> Vec<(String, f32)> {
        self.components.refresh(true);
        self.components
            .iter()
            .filter_map(|c| c.temperature().map(|t| (c.label().to_string(), t)))
            .collect()
    }
}
```

The caller creates `SensorReader::new()` once, then calls `.read()` on each tick. The `Components` object lives inside the struct and is never recreated.

Notice `&mut self` on `read()` — refreshing updates the struct's internal state, so Rust requires it to be mutable. See [rust_basics.md](../rust/rust_basics.md) for the distinction between `&self` and `&mut self`.

---

## Reading directly from the kernel (no crate)

`sysinfo` reads from Linux's `hwmon` subsystem under the hood. You can do it directly using only the standard library — useful for understanding what is happening, or for zero-dependency builds.

### The paths

```
/sys/class/hwmon/hwmon*/name          → driver name (e.g. "coretemp" or "k10temp")
/sys/class/hwmon/hwmon*/temp1_input   → temperature in millidegrees Celsius
/sys/class/hwmon/hwmon*/temp1_label   → human label (e.g. "Package id 0")
/sys/class/hwmon/hwmon*/temp2_input   → next sensor
/sys/class/hwmon/hwmon*/temp2_label
```

The raw temperature value is in **millidegrees Celsius** — divide by 1000 to get °C. `44000` → `44.0 °C`.

Driver names by CPU brand:

| Driver | CPU family |
|---|---|
| `coretemp` | Intel Core (all generations) |
| `k10temp` | AMD Ryzen / EPYC (Zen, Zen 2+) |

### Standard library tools

```rust
use std::fs;
use std::path::PathBuf;

// read a file to a String
let content = fs::read_to_string("/path/to/file")?;

// trim whitespace (sysfs files end with a newline)
let clean = content.trim();

// parse a string into a number
let millidegrees: i64 = clean.parse()?;
let celsius = millidegrees as f32 / 1000.0;

// list entries in a directory
let entries = fs::read_dir("/sys/class/hwmon")?;

// build a path
let base = PathBuf::from("/sys/class/hwmon/hwmon0");
let name_file = base.join("name");
```

### Exploring your system first

Before writing any code, run these in your terminal to see what your machine exposes:

```bash
# list all hwmon drivers
for f in /sys/class/hwmon/hwmon*/; do
  echo "$(basename $f): $(cat $f/name)"
done

# read a temperature directly
cat /sys/class/hwmon/hwmon0/temp1_input
```

---

## Further reading

- [sysinfo docs](https://docs.rs/sysinfo/latest/sysinfo/)
- [rust_patterns.md](../rust/rust_patterns.md) — `Option` handling for `.temperature()`
- [rust_basics.md](../rust/rust_basics.md) — `for` loops, `if let`, `mut`
- [ideas/what-to-measure.md](../ideas/what-to-measure.md) — what else sysinfo exposes beyond CPU temperature
