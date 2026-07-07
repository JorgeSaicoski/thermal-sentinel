use std::thread;
use sysinfo::{Component, Components, System};
use crate::domain::cpu_info::CpuInfo;

pub fn read() -> CpuInfo {
    let components = Components::new_with_refreshed_list();
    let temperature = components
        .iter()
        .find_map(|c: &Component| c.temperature())
        .unwrap_or(0.0);

    let mut sys = System::new();
    sys.refresh_cpu_usage();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    let usage = sys.global_cpu_usage();

    CpuInfo { temperature, usage }
}