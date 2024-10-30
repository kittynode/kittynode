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
    InstallPackage {
        #[arg(value_name = "PACKAGE_NAME")]
        name: String,
    },
    DeletePackage {
        #[arg(value_name = "PACKAGE_NAME")]
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli: Cli = Cli::parse();
    match cli.command {
        Commands::GetPackages => {
            commands::get_packages_command().await?;
        }
        Commands::InstallPackage { name } => {
            commands::install_package(name).await?;
        }
        Commands::DeletePackage { name } => {
            commands::delete_package(name).await?;
        }
    }
    Ok(())
}
