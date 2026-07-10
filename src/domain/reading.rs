use crate::domain::cpu_info::{CpuInfo, CpuDetailInfo};
pub struct ReadCPU {          // pub: other modules can use this type
    pub timestamp: String,    // pub: other modules can read this field
    pub cpu: CpuInfo,
}

pub struct ReadAllCPU {
    pub timestamp: String,
    pub cpus: Vec<CpuInfo>,
}

pub struct ReadAllCPUDetail {
    pub timestamp: String,
    pub vendor: String,
    pub brand: String,
    pub cpus: Vec<CpuDetailInfo>,
}