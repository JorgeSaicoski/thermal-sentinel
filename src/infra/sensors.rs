use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{Component, Components, CpuRefreshKind, System, RefreshKind};
use crate::domain::cpu_info::{CpuInfo, CpuDetailInfo};
use crate::domain::reading::ReadAllCPUDetail;
const DEFAULT_TEMPERATURE: f32 = 0.0;

pub struct SensorReader{
    components: Components,
}

impl SensorReader {
    pub fn new() -> Self {
        let mut components = Components::new_with_refreshed_list();
        components.refresh(true);
        Self { components }
    }
    pub fn read_cpu(&self) -> CpuInfo {
        let temperature = self.read_cpu_temperature();

        CpuInfo {
            label: "Average".to_string(),
            temperature,
        }
    }

    pub fn read_all_cpu(&self) -> Vec<CpuInfo> {
        let mut readings:Vec<CpuInfo> = Vec::new();
        for c in &self.components {
            let label = c.label().to_string();
            let temperature = c.temperature().unwrap_or(0.0);
            readings.push(CpuInfo { label, temperature });
        }
        readings
    }

    pub fn global_usage(&self) -> f32 {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_all();
        sys.global_cpu_usage()
    }

    fn read_cpu_temperature(&self) -> f32 {
        self.components
            .iter()
            .find_map(|component: &Component| component.temperature())
            .unwrap_or(DEFAULT_TEMPERATURE)
    }

    pub fn read_all_cpu_detail(&self) -> ReadAllCPUDetail {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        // Refresh CPUs again to get actual value.
        sys.refresh_cpu_all();

        let mut cpus = Vec::new();
        let mut brand = String::new();
        let mut vendor = String::new();

        if let Some(first_cpu) = sys.cpus().first() {
            brand = first_cpu.brand().to_string();
            vendor = first_cpu.vendor_id().to_string();
        }

        for cpu in sys.cpus() {
            cpus.push(CpuDetailInfo {
                name: cpu.name().to_string(),
                frequency: cpu.frequency() as f32,
                usage: cpu.cpu_usage(),
            });
        }

        ReadAllCPUDetail {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
            vendor,
            brand,
            cpus,
        }
    }
}
