use clap::{Subcommand, Parser};

#[derive(Parser)]
#[command(name = "thermal-sentinel")]
#[command(about = "A thermal sensor monitoring tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    Avg {
        #[arg(short, long, default_value_t = 2)]
        interval: u64,
    },
    Detail {
        #[arg(short, long, default_value_t = 10)]
        interval: u64,
    },
    Info {
        #[arg(short, long, default_value_t = 10)]
        interval: u64,
    },
    Score{
        #[arg(short, long, default_value_t = 10)]
        interval: u64,
    },
    Snapshot,

}