mod memhog;
mod clicommands;

fn main() {
    let args = clicommands::Cli::parse();

    match args.command {
        clicommands::Commands::Memhog { megabytes, seconds, remove_safety } => {
            memhog::memhog::hogger(megabytes, seconds, !remove_safety);
        }
    }
}
