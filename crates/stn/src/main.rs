mod commands;
mod constants;
mod docker;
mod env_manager;
mod network;

use std::{
    env, io,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a Taiko node
    Install,
    /// Configure your Taiko node
    Config(Config),
    /// Start your Taiko node
    Up,
    /// Stop your Taiko node
    Down,
    /// Upgrade your Taiko node
    Upgrade,
    /// Restarts your Taiko node
    Restart,
    /// Delete your Taiko node
    Remove,
    /// Logs of your Taiko node
    Logs(Logs),
    /// Status of your Taiko node
    Status,
    /// System usage stats of your Taiko node
    Stats,
    /// Open the Grafana dashboard in default browser
    Dashboard,
}

#[derive(Parser)]
struct Logs {
    #[command(subcommand)]
    subcommands: LogsSubcommands,
}

#[derive(Parser)]
struct Config {
    #[command(subcommand)]
    subcommands: ConfigSubcommands,
}

#[derive(Subcommand)]
pub enum LogsSubcommands {
    /// Shows all logs
    All,
    /// Shows execution logs
    Execution,
    /// Shows driver logs
    Driver,
    /// Show proposer logs
    Proposer,
}

#[derive(Subcommand)]
pub enum ConfigSubcommands {
    /// Config a full node
    Full,
    /// Config a proposer
    Proposer,
    /// Config a ZK prover
    Zkp,
    /// Config an SGX prover
    Sgx,
}

#[tokio::main]
async fn main() {
    let taiko_node_dir = match get_stn_directory() {
        Ok(dir) => dir.join(constants::TAIKO_NODE_DIRECTORY_NAME),
        Err(e) => {
            eprintln!("Error getting Taiko node directory: {}", e);
            return;
        }
    };

    let cli: Cli = Cli::parse();

    match &cli.command {
        Commands::Install => {
            commands::install(&taiko_node_dir);
        }
        Commands::Config(config_subcommands) => {
            commands::config(&config_subcommands.subcommands, &taiko_node_dir).await;
        }
        Commands::Up => {
            commands::up(&taiko_node_dir).await;
        }
        Commands::Down => {
            commands::down(&taiko_node_dir);
        }
        Commands::Upgrade => {
            commands::upgrade(&taiko_node_dir).await;
        }
        Commands::Restart => {
            commands::restart(&taiko_node_dir).await;
        }
        Commands::Remove => {
            commands::remove(&taiko_node_dir);
        }
        Commands::Logs(logs_subcommands) => {
            commands::logs(&logs_subcommands.subcommands, &taiko_node_dir);
        }
        Commands::Status => {
            commands::status(&taiko_node_dir).await;
        }
        Commands::Stats => {
            commands::stats(&taiko_node_dir);
        }
        Commands::Dashboard => {
            commands::dashboard(&taiko_node_dir);
        }
    }
}

/// Helper function that returns the path to the stn config directory.
pub fn get_stn_directory() -> Result<PathBuf, io::Error> {
    let home_dir = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Failed to get home directory"))?;
    Ok(Path::new(&home_dir).join(constants::STN_PATH))
}
