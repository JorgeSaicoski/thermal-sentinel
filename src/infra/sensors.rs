use sysinfo::{Component, Components, CpuRefreshKind, System, RefreshKind};
use crate::domain::cpu_info::CpuInfo;

const DEFAULT_TEMPERATURE: f32 = 0.0;

pub fn read_cpu() -> CpuInfo {
    let temperature = read_cpu_temperature();
    let label = "Average".to_string();

    CpuInfo {
        label,
        temperature,
    }
}

fn read_cpu_temperature() -> f32 {
    let components = Components::new_with_refreshed_list();

    components
        .iter()
        .find_map(|component: &Component| component.temperature())
        .unwrap_or(DEFAULT_TEMPERATURE)
}

pub fn read_all_cpu() -> Vec<CpuInfo> {
    let components = Components::new_with_refreshed_list();
    let mut readings = Vec::new();
    for c in &components {
        let label = c.label().to_string();
        let temperature = c.temperature().unwrap_or(0.0);
        readings.push(CpuInfo { label, temperature });
    }
    readings   
}

pub fn read_all_cpu_detail(){
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    s.refresh_cpu_all();
    println!("{}", s.cpus()[0].brand());
    println!("{}", s.cpus()[0].vendor_id());
    for cpu in s.cpus() {
        println!("{}%", cpu.cpu_usage());
        println!("{}", cpu.name());
        println!("{}", cpu.frequency());
    }
}