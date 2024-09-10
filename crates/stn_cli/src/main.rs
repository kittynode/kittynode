use clap::{Parser, Subcommand};
use stn_core::install;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install,
}

#[tokio::main]
async fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Install => {
            install();
        }
    }
}
