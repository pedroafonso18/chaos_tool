mod hoggers;
mod clicommands;
mod database;

fn main() {
    let args = clicommands::Cli::parse();

    match args.command {
        clicommands::Commands::Memhog { megabytes, seconds, remove_safety } => {
            hoggers::memhog::memhogger(megabytes, seconds, !remove_safety);
        },
        clicommands::Commands::Cpuhog { cores, seconds, remove_safety } => {
            hoggers::cpuhog::cpuhogger(cores, seconds, !remove_safety);
        },
        clicommands::Commands::Dbfull { users, seconds, remove_safety, dburl } => {
            database::dbfull::dbfull(&dburl, users, seconds, remove_safety);
        },
        clicommands::Commands::Dbfailure { remote_host, remote_port, seconds } => {
            database::dbfailure::dbfailure(&remote_host, remote_port, seconds);
        }
    }
}
