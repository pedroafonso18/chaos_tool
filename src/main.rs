mod hoggers;
mod clicommands;
mod database;
mod vm;
mod docker;

fn main() {
    let args = clicommands::Cli::parse();

    let result = match args.command {
        clicommands::Commands::Memhog { megabytes, seconds, remove_safety } => {
            hoggers::memhog::memhogger(megabytes, seconds, !remove_safety)
        },
        clicommands::Commands::Cpuhog { cores, seconds, remove_safety } => {
            hoggers::cpuhog::cpuhogger(cores, seconds, !remove_safety)
        },
        clicommands::Commands::Dbfull { users, seconds, remove_safety, dburl } => {
            database::dbfull::dbfull(&dburl, users, seconds, remove_safety)
        },
        clicommands::Commands::Dbfailure { remote_host, remote_port, seconds } => {
            database::dbfailure::dbfailure(&remote_host, remote_port, seconds)
        },
        clicommands::Commands::Vmfailure { remote_host, seconds } => {
            vm::vmfailure::vmfailure(&remote_host, seconds)
        },
        clicommands::Commands::Dockerkill { container_name, is_random, prune } => {
            docker::dockerkill::dockerkill(container_name, is_random, prune)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
