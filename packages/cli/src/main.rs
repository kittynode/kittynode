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
        #[arg(long = "include-images", help = "Whether to include Docker images")]
        include_images: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    match Cli::parse().command {
        Commands::GetPackages => {
            commands::get_packages_command().await?;
        }
        Commands::InstallPackage { name } => {
            commands::install_package(name).await?;
        }
        Commands::DeletePackage {
            name,
            include_images,
        } => {
            commands::delete_package(name, include_images).await?;
        }
    }
    Ok(())
}
