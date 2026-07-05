# What to Measure — sysinfo Beyond CPU Temperature

The minimal startup doc uses CPU temperature as its example. But `sysinfo` can give you much more. This document lists what is available and what questions each measurement can answer.

Nothing here tells you what to track. That depends on what you want to know.

---

## CPU

```rust
use sysinfo::{Components, System};
```

| Data | How to get it | What you can ask |
|---|---|---|
| Temperature per sensor | `Components` + `.temperature()` | Which core runs hottest? |
| Global CPU usage | `System` + `.global_cpu_usage()` | How busy is the CPU overall? |
| Per-core usage | `System` + `.cpus()` → `.cpu_usage()` | Is load balanced across cores? |
| CPU brand/model | `System` + `.global_cpu_info().brand()` | What machine is this? |

Temperature and usage together can tell you more than either alone. A hot CPU at low usage suggests thermal throttling or poor airflow. A cool CPU at high usage suggests the workload is well-distributed.

---

## Memory

```rust
use sysinfo::System;

let mut sys = System::new_all();
sys.refresh_memory();

let total = sys.total_memory();       // bytes
let used  = sys.used_memory();        // bytes
let free  = sys.free_memory();        // bytes
let swap_total = sys.total_swap();
let swap_used  = sys.used_swap();
```

All values are in bytes — divide by `1_073_741_824.0` to get GB.

What you can ask: Is memory pressure contributing to performance degradation? Is swap being used, which usually means RAM is exhausted?

---

## Disks

```rust
use sysinfo::Disks;

let disks = Disks::new_with_refreshed_list();

for disk in &disks {
    println!("{}: {} GB free of {} GB",
        disk.name().to_string_lossy(),
        disk.available_space() / 1_073_741_824,
        disk.total_space()     / 1_073_741_824,
    );
}
```

What you can ask: Is a disk nearly full? Which mount point is running low?

---

## Networks

```rust
use sysinfo::Networks;

let networks = Networks::new_with_refreshed_list();

for (name, data) in &networks {
    println!("{}: ↓ {} B  ↑ {} B",
        name,
        data.received(),
        data.transmitted(),
    );
}
```

Values are cumulative since the interface came up — to get rate, take two readings with a sleep between them and subtract.

What you can ask: Is this machine sending or receiving unusual traffic? Is one interface busy?

---

## Processes

```rust
use sysinfo::System;

let mut sys = System::new_all();
sys.refresh_all();

for (pid, process) in sys.processes() {
    println!("{}: {} — {:.1}% CPU, {} MB",
        pid,
        process.name().to_string_lossy(),
        process.cpu_usage(),
        process.memory() / 1_048_576,
    );
}
```

What you can ask: Which process is consuming the most CPU right now? Is a specific application leaking memory over time?

---

## System information

```rust
use sysinfo::System;

System::host_name()       // Option<String>
System::os_version()      // Option<String>
System::long_os_version() // Option<String>
System::uptime()          // u64 — seconds since boot
```

What you can ask: How long has this machine been running without a restart?

---

## Putting it together

You decide which of these to include in your `Reading` struct. Each one follows the same pattern as `CpuInfo`: define a type in `domain/`, write an adapter in `infra/`, add the field to `Reading`, display it in `interface/`.

If you want to track memory alongside CPU, the shape would be:

```rust
// domain/memory_info.rs
pub struct MemoryInfo {
    pub used_gb: f32,
    pub total_gb: f32,
}
```

Same pattern as `CpuInfo`. Same adapter structure in `infra/`. Same placement in `Reading`.

---

## Further reading

- [crates/sysinfo.md](../crates/sysinfo.md) — temperature and usage in detail
- [architecture/minimal-startup.md](../architecture/minimal-startup.md) — the pattern for adding any new measurement
