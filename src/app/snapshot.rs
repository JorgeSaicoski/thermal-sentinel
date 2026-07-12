use std::time::{SystemTime, UNIX_EPOCH};
use crate::domain::reading::{ReadCPU, ReadAllCPU, ReadAllCPUDetail};
use crate::infra::sensors::SensorReader;

pub fn take(sensor_reader: &mut SensorReader) -> ReadCPU{
    let sensor = sensor_reader.read_cpu();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();

    ReadCPU { timestamp, cpu: sensor }
}

pub fn take_all(sensor_reader: &mut SensorReader) -> ReadAllCPU{
    let cpus = sensor_reader.read_all_cpu();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();
    ReadAllCPU{timestamp, cpus}
}

pub fn take_all_detail(sensor_reader: &mut SensorReader) -> ReadAllCPUDetail{
    sensor_reader.read_all_cpu_detail()
}
