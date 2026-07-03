
pub struct Reading {          // pub: other modules can use this type
    pub timestamp: String,    // pub: other modules can read this field
    pub cpu_temp: f32,
    pub cpu_usage: f32,
    pub outdoor_temp: Option<f32>,
    pub city: Option<String>,
}