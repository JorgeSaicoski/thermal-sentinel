use crate::domain::reading::{ReadCPU, ReadAllCPU};

pub fn display_reading(reading: ReadCPU){
    println!(
        "[{}] CPU: {:.1} °C",
        reading.timestamp, reading.cpu.temperature
    );
}

pub fn display_readings(readings: ReadAllCPU){
    println!("All CPUs:");
    println!("timestamp: {}", readings.timestamp);
    for cpu in &readings.cpus {
        println!(
            "[{}] Temp: {:.1} °C",
            cpu.label, cpu.temperature
        )
    }
}

