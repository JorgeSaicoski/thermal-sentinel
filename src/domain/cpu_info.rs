pub struct CpuInfo {
    pub label: String,
    pub temperature: f32,
}

pub struct CpuDetailInfo {
    pub name: String,
    pub frequency: f32,
    pub usage: f32,
}

impl CpuInfo {
    pub fn hottest(temps: &[CpuInfo]) -> Option<f32> {
        temps.iter().map(|c| c.temperature).reduce(f32::max)
    }
}