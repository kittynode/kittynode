use clap::{Parser, Subcommand};
use eyre::Result;
use kittynode_core::{check_docker_version, install};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Install,
    Check,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse();
    match cli.command {
        Commands::Install => {
            install()?;
        }
        Commands::Check => {
            check_docker_version().await?;
        }
    }
    Ok(())
}
