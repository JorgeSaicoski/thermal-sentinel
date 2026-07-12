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
        none => {
            let reading = app::snapshot::take();
            interface::display::display_reading(reading)
        }
        Some(Commands::Avg {interval}) =>{
            
            let reading = app::snapshot::take();
            interface::display::display_reading(reading)
        }
    }
}

