use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::reading::{ReadCPU, ReadAllCPU, ReadAllCPUDetail};
use crate::infra::sensors;

pub fn take() -> ReadCPU{
    let cpu = sensors::read_cpu();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();

    ReadCPU{timestamp, cpu}
}

pub fn take_all() -> ReadAllCPU{
    let cpus = sensors::read_all_cpu();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();
    ReadAllCPU{timestamp, cpus}
}

pub fn take_all_detail() -> ReadAllCPUDetail{
    sensors::read_all_cpu_detail()
}