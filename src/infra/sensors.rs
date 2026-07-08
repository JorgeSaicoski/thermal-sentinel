use std::thread;
use sysinfo::{Component, Components, System};
use crate::domain::cpu_info::CpuInfo;

const DEFAULT_TEMPERATURE: f32 = 0.0;

pub fn read_cpu() -> CpuInfo {
    let temperature = read_cpu_temperature();
    let usage = read_global_cpu_usage();
    let label = "Average".to_string();

    CpuInfo {
        label,
        temperature,
        usage,
    }
}

fn read_cpu_temperature() -> f32 {
    let components = Components::new_with_refreshed_list();

    components
        .iter()
        .find_map(|component: &Component| component.temperature())
        .unwrap_or(DEFAULT_TEMPERATURE)
}

fn read_global_cpu_usage() -> f32 {
    let mut system = System::new();

    system.refresh_cpu_usage();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu_usage();

    system.global_cpu_usage()
}

pub fn read_all_cpu() -> Vec<CpuInfo> {
    let components = Components::new_with_refreshed_list();
    let mut readings = Vec::new();
    for c in &components {
        let label = c.label().to_string();
        let temperature = c.temperature().unwrap_or(0.0);
        let usage = read_global_cpu_usage();
        readings.push(CpuInfo { label, temperature, usage });
    }
    readings   
}