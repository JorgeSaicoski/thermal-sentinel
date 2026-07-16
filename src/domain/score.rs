pub fn compute(cpu_temp: f32, avg_usage: f32, external_temp: f32) -> f32 {
    let temp_diff = cpu_temp - external_temp;
    let usage_weight = (avg_usage + 1.0).log10() / 10.1;
    let score = temp_diff * usage_weight;
    score
}