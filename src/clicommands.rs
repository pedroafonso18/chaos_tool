use clap::{Parser, Subcommand};


#[derive(Subcommand)]
pub enum Commands {
    /// Simulate high RAM usage
    Memhog {
        /// Amount of memory to allocate in megabytes (e.g., 1024 for 1GB)
        #[clap(short, long)]
        megabytes: u32,
        /// Number of seconds to run the simulation
        #[clap(short, long)]
        seconds: u32,
        /// Remove safety checks (not recommended unless you know what you're doing)
        #[clap(short, long)]
        remove_safety: bool,
    },
    /// Simulate high CPU usage
    Cpuhog {
        /// Amount of CPU cores to allocate (e.g., 2, 1)
        #[clap(short, long)]
        cores: u32,
        /// Number of seconds to alocate for
        #[clap(short, long)]
        seconds: u32,
        /// Remove safety checks (ESPECIALLY NOT RECOMMENDED, THIS MAY SHUTDOWN YOUR PC)
        #[clap(short, long)]
        remove_safety: bool,
    },
}


#[derive(Parser)]
#[command(name = "chaostool", about = "Generate controlled chaos in your environment!")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn parse() -> Self {
        Cli::parse_from(std::env::args())
    }
}