use crate::domain::reading::Reading;

pub fn show(reading: &Reading) {
    println!(
        "[{}] CPU: {:.1} °C | Usage: {:.1}%",
        reading.timestamp, reading.cpu.temperature, reading.cpu.usage
    );
}