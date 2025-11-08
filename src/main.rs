use clap::{Parser, Subcommand};

use rito_cli::{Result, commands};

#[derive(Parser, Debug)]
#[command(
    name = "rito-cli",
    version,
    about = "Rito command-line interface",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Goal,
    Task,
    Habit,
    Journal,
    Report,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Goal => commands::goal::run(),
        Commands::Task => commands::task::run(),
        Commands::Habit => commands::habit::run(),
        Commands::Journal => commands::journal::run(),
        Commands::Report => commands::report::run(),
    }
}
