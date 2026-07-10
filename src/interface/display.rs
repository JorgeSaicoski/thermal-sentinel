use crate::domain::reading::{ReadCPU, ReadAllCPU, ReadAllCPUDetail};

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

pub fn display_readings_detail(readings:ReadAllCPUDetail){
    println!("All CPUs Details:");
    println!("timestamp {}", readings.timestamp);
    println!("vendor {}", readings.vendor);
    println!("brand {}", readings.brand);
    for cpu in readings.cpus {
        println!("name: {}", cpu.name);
        println!("frequency: {}", cpu.frequency);
        println!("usage: {}", cpu.usage);
    }
}