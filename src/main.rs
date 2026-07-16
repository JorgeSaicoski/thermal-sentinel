use std::io::{self, Write};
use clap::Parser;
use crate::cli::Commands;
use crate::domain::cpu_info::CpuInfo;

mod app;
mod domain;
mod infra;
mod interface;
mod cli;

fn main()  {
    let args = cli::Cli::parse();
    match args.command {
        None => {
            let mut reader = infra::sensors::SensorReader::new();
            let reading = app::snapshot::take(&mut reader);
            interface::display::display_reading(reading);
        }
        Some(Commands::Avg { interval }) => {
            let mut reader = infra::sensors::SensorReader::new();
            loop {
                let reading = app::snapshot::take(&mut reader);
                interface::display::display_reading(reading);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }

        }
        Some(Commands::Detail { interval }) => {
            let mut reader = infra::sensors::SensorReader::new();
            loop {
                let reading = app::snapshot::take_all_detail(&mut reader);
                interface::display::display_readings_detail(reading);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        }
        Some(Commands::Info { interval }) => {
            let mut reader = infra::sensors::SensorReader::new();
            loop {
                let reading = app::snapshot::take_all(&mut reader);
                interface::display::display_readings(reading);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        }
        Some(Commands::Score { interval }) => {
            let mut reader = infra::sensors::SensorReader::new();
            let mut external_temp: f32 = ask_user_for_temp();
            let mut count: u64 = 0;
            loop {
                if interval * count >= 3600 {
                    external_temp = ask_user_for_temp();
                    count = 0;
                }

                let temps = reader.read_all_cpu();
                let hottest = CpuInfo::hottest(&temps).unwrap_or(0.0);
                let usage   = reader.global_usage();
                let score   = domain::score::compute(hottest, usage, external_temp);
                println!("External Temp: {}°C, CPU Temp: {}°C, CPU Usage: {:.2}%, Score: {:.2}", external_temp, hottest, usage, score);
                println!("Score: {}", score);
                count += 1;
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        }
        Some(Commands::Snapshot) => {
            let mut reader = infra::sensors::SensorReader::new();
            let reading = app::snapshot::take(&mut reader);
            interface::display::display_reading(reading)
        }
    }
}

fn ask_user_for_temp() -> f32 {
    print!("Enter external temperature (°C): ");
    std::io::stdout().flush().unwrap();      // flush so the prompt appears before input

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read input");

    input.trim().parse::<f32>().unwrap_or(0.0)
}