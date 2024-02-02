mod commands;
mod constants;
mod docker;
mod env_manager;
mod network;
mod update_checker;
mod utils;

use clap::{Parser, Subcommand};
use update_checker::UpdateChecker;

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
enum LogsSubcommands {
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
enum ConfigSubcommands {
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
    let cli = Cli::parse();

    let taiko_node_dir = match utils::get_stn_directory() {
        Ok(dir) => dir.join(constants::TAIKO_NODE_DIRECTORY_NAME),
        Err(e) => {
            eprintln!("Error getting Taiko node directory: {}", e);
            return;
        }
    };

    let stn_dir = match utils::get_stn_directory() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting stn directory: {}", e);
            return;
        }
    };

    // Check for updates, pulling from a cache
    if let Err(e) = UpdateChecker::new(stn_dir).check_for_updates().await {
        eprintln!("Failed to check for updates: {}", e);
    }

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
