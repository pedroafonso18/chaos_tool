mod hoggers;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();

    match args.command {
        clicommands::Commands::Memhog { megabytes, seconds, remove_safety } => {
            hoggers::memhog::memhogger(megabytes, seconds, !remove_safety);
        },
        clicommands::Commands::Cpuhog { cores, seconds, remove_safety } => {
            hoggers::cpuhog::cpuhogger(cores, seconds, !remove_safety);
        }
    }
}
