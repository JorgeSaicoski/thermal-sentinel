use crate::domain::cpu_info::CpuInfo;
pub struct ReadCPU {          // pub: other modules can use this type
    pub timestamp: String,    // pub: other modules can read this field
    pub cpu: CpuInfo,
}

pub struct ReadAllCPU {
    pub timestamp: String,
    pub cpus: Vec<CpuInfo>,
}