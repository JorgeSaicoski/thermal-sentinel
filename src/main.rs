use clap::Parser;
use crate::cli::Commands;

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
        Some(Commands::Detail { limit: _, interval: _ }) => {
            let mut reader = infra::sensors::SensorReader::new();
            let reading = app::snapshot::take_all_detail(&mut reader);
            interface::display::display_readings_detail(reading)
        }
        Some(Commands::Info { limit: _, interval: _ }) => {
            let mut reader = infra::sensors::SensorReader::new();
            let reading = app::snapshot::take_all(&mut reader);
            interface::display::display_readings(reading)
        }
        Some(Commands::Snapshot) => {
            let mut reader = infra::sensors::SensorReader::new();
            let reading = app::snapshot::take(&mut reader);
            interface::display::display_reading(reading)
        }
    }
}
