use clap::{Parser, Subcommand};
use eyre::Result;
mod commands;

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetPackages,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    match cli.command {
        Commands::GetPackages => {
            commands::get_packages_command().await?;
        }
    }
    Ok(())
}
