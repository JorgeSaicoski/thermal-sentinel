use crate::domain::reading::{ReadCPU, ReadAllCPU};

pub fn display_reading(reading: ReadCPU){
    println!(
        "[{}] CPU: {:.1} °C | Usage: {:.1}%",
        reading.timestamp, reading.cpu.temperature, reading.cpu.usage
    );
}

pub fn display_readings(readings: ReadAllCPU){
    println!("All CPUs:");
    println!("timestamp: {}", readings.timestamp);
    for cpu in &readings.cpus {
        println!(
            "[{}] Temp: {:.1} °C | Usage: {:.1}",
            cpu.label, cpu.temperature, cpu.usage
        )
    }
}

