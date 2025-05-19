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

    /// Simulate max users at db
    Dbfull {
        // Amount of users you want to insert at the db
        #[clap(short, long)]
        users: u32,

        // Amount of time the users will be active for
        #[clap(short, long)]
        seconds: u32,

        // Remove safety checks (May cause connecting to database impossible)
        #[clap(short, long)]
        remove_safety:bool,

        // Db Url for connecting to database
        #[clap(value_name = "PATTERN")]
        dburl: String,
    },

    /// Simulate DB failure. WARNING: This is extremely unsafe by nature, so no safety tag, use at your own risk.
    Dbfailure {
        #[clap(value_name = "PATTERN")]
        /// Remote host to connect to, be sure to pass the root user as the parameter.
        remote_host: String,
        #[clap(short, long)]
        /// Remote host of the db, if it is postgres usually it will be in port 5432.
        remote_port: i32,
        /// The seconds to block the use of the db for.
        #[clap(short, long)]
        seconds: i32,
    }
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