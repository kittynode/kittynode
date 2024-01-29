mod constants;
mod docker;
mod env_manager;
mod network;
mod utils;

use clap::{Parser, Subcommand};
use env_manager::EnvManager;
use network::get_sync_state;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, io};
use utils::stn_log;

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

    let taiko_node_dir = match utils::get_taiko_node_directory() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting Taiko node directory: {}", e);
            return;
        }
    };

    match &cli.command {
        Commands::Install => {
            install(&taiko_node_dir);
        }
        Commands::Config(config_subcommands) => {
            config(&config_subcommands.subcommands, &taiko_node_dir).await;
        }
        Commands::Up => {
            up(&taiko_node_dir);
        }
        Commands::Down => {
            down(&taiko_node_dir);
        }
        Commands::Upgrade => {
            upgrade(&taiko_node_dir);
        }
        Commands::Restart => {
            restart(&taiko_node_dir);
        }
        Commands::Remove => {
            remove(&taiko_node_dir);
        }
        Commands::Logs(logs_subcommands) => {
            logs(&logs_subcommands.subcommands, &taiko_node_dir);
        }
        Commands::Status => {
            status(&taiko_node_dir).await;
        }
    }
}

fn install(taiko_node_dir: &Path) {
    // Check if Taiko node is already installed
    if taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is already installed.");
        return;
    }

    utils::stn_log(&format!(
        "Installing simple-taiko-node to {}",
        taiko_node_dir.to_str().unwrap()
    ));

    // Create home directory if it doesn't exist
    fs::create_dir_all(taiko_node_dir).expect("Failed to create .stn directory");

    // Pull latest simple-taiko-node from GitHub
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(constants::SIMPLE_TAIKO_NODE_REPO_URL)
        .arg(taiko_node_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute git clone command.");

    let git_clone_status = git_clone
        .wait()
        .expect("Failed to wait for git clone to complete.");

    if git_clone_status.success() {
        utils::stn_log("Git clone successful.");
    } else {
        utils::stn_log("Git clone failed.");
    }

    // Copy .env.sample to .env
    std::fs::copy(
        Path::new(&taiko_node_dir).join(".env.sample"),
        Path::new(&taiko_node_dir).join(".env"),
    )
    .expect("Failed to copy .env.sample to .env");

    utils::stn_log("simple-taiko-node successfully installed");
}

async fn config(config_subcommand: &ConfigSubcommands, taiko_node_dir: &Path) {
    match config_subcommand {
        ConfigSubcommands::Full => {
            let mut l1_endpoint_http = String::new();
            let mut l1_endpoint_ws = String::new();

            // Ask the user if they have an L1_ENDPOINT_HTTP and L1_ENDPOINT_WS
            let mut input = String::new();
            print!("Do you have an HTTP and WS endpoint for a Holesky L1 archive node? (y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            if input.trim() != "y" {
                println!(
                        concat!("\nYou must have an HTTP and WS endpoint for a Holesky L1 archive node to configure a Taiko node. You can either:\n",
                                "  1. Install a Holesky L1 archive node and run it locally\n",
                                "  2. Use a public Holesky L1 archive node from an RPC provider (will have to pay or eventually get rate limited)\n",
                                "\nSee the docs at https://docs.taiko.xyz/guides/run-a-taiko-node for more info."));
                return;
            }

            print!("Please enter your L1_ENDPOINT_HTTP: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut l1_endpoint_http)
                .expect("Failed to read L1_ENDPOINT_HTTP");

            print!("Please enter your L1_ENDPOINT_WS: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut l1_endpoint_ws)
                .expect("Failed to read L1_ENDPOINT_WS");

            l1_endpoint_http = l1_endpoint_http.trim().to_string();
            l1_endpoint_ws = l1_endpoint_ws.trim().to_string();

            let (http_valid, ws_valid) =
                network::validate_endpoints(&l1_endpoint_http, &l1_endpoint_ws).await;

            if http_valid && ws_valid {
                println!("Both HTTP and WS endpoints are valid.");
            } else {
                println!("One or both of the endpoints are invalid.");
                return; // Don't continue if endpoints are invalid
            }

            // Initialize EnvManager and update .env with L1_ENDPOINT_HTTP and L1_ENDPOINT_WS
            let env_path = taiko_node_dir.join(".env");
            let mut env_manager =
                EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

            env_manager.set("L1_ENDPOINT_HTTP".to_string(), l1_endpoint_http);
            env_manager.set("L1_ENDPOINT_WS".to_string(), l1_endpoint_ws);
            env_manager.save().expect("Failed to save .env file");

            println!("simple-taiko-node successfully configured.");
        }
        // there are basically two prerequisites for becoming a proposer:
        // 1. you need to have a prover currently running OR a marketplace prover configured. (i have a default static endpoint they can use, which we can tell them)
        // 2. you need to have a node installed, running, and fully synced
        // at the end i think we should ask the user if you'd like the tool to restart the node.
        //
        // algorithm:
        // 0. check if they have a node installed, running, and fully synced, if so proceed, if not, send the error message
        // 1. check if they have a local prover running, if so, verify the prover API is functional
        // 2. if they don't have a local prover running, ask if they would like to setup a marketplace prover
        // 3. if they say yes, healthcheck the marketplace prover
        // 4. if the marketplace prover fails, send them to the docs to find another marketplace prover
        // 5. if the marketplace prover succeeds, set the variable in their .env with env_manager
        // 6. now that their prover is set, and functional, and we have verified they are fully synced, proceed to enable_proposer flag to true
        // 7. ask the user if they would like us to restart the node for them
        ConfigSubcommands::Proposer => {
            // Initialize EnvManager and var to track intent
            let env_path = taiko_node_dir.join(".env");
            let mut env_manager =
                EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

            // Check current state of the proposer
            let current_state = env_manager
                .get("ENABLE_PROPOSER")
                .unwrap_or("false".to_string());

            if current_state != "true" {
                println!("The node is currently not configured as a proposer. Would you like to enable it? (y/n): ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                if input.trim() == "y" {
                    // Enable proposer
                    // 0. Check if they have a node installed, running, and fully synced
                    let local_http = format!(
                        "http://localhost:{}",
                        env_manager
                            .get("PORT_L2_EXECUTION_ENGINE_HTTP")
                            .expect("PORT_L2_EXECUTION_ENGINE_HTTP not set")
                    );
                    let canonical_http = constants::KATLA_RPC_URL.to_string();
                    let node_synced = network::is_synced(&local_http, &canonical_http).await;
                    if !node_synced {
                        println!("Node is not installed, running, or fully synced.");
                        return;
                    }
                    // 1. Check if they have a local prover running [TODO: refactor into taiko_node::prover::verify_prover_api()]
                    let local_prover_running = env_manager
                        .get("PROVER_ENDPOINTS")
                        .expect("PROVER_ENDPOINTS not set.")
                        .to_string()
                        .contains("taiko_client_prover_relayer");
                    if local_prover_running {
                        // Check if local prover is functional
                        if !network::is_prover_api_functional(&local_http).await {
                            println!("Local prover is running but the API is not functional.");
                            return;
                        }
                    } else {
                        // 2. If they don't have a local prover running, ask if they would like to setup a marketplace prover
                        println!("Would you like to setup a marketplace prover? (y/n): ");
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input");
                        if input.trim() == "y" {
                            // 3. Healthcheck the marketplace prover
                            if !network::is_prover_api_functional(constants::DEFAULT_PROVER_URL)
                                .await
                            {
                                // 4. If the marketplace prover fails, send them to the docs to find another marketplace prover
                                println!("Marketplace prover failed. Please consult the documentation to find another marketplace prover.");
                                return;
                            } else {
                                // 5. If the marketplace prover succeeds, set the variable in their .env with env_manager
                                env_manager.set(
                                    "PROVER_ENDPOINTS".to_string(),
                                    constants::DEFAULT_PROVER_URL.to_string(),
                                );
                            }
                        } else {
                            println!(
                                "Local prover not running and marketplace prover setup declined."
                            );
                            return;
                        }
                    }
                    // 6. Now that their prover is set, and functional, and we have verified they are fully synced, proceed to enable_proposer flag to true
                    env_manager.set("ENABLE_PROPOSER".to_string(), "true".to_string());
                    env_manager.save().expect("Failed to save .env file");
                } else {
                    println!("No changes made to proposer configuration.");
                    return;
                }
            } else {
                println!("The node is currently configured as a proposer. Would you like to disable it? (y/n): ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                if input.trim() == "y" {
                    // Disable proposer
                    env_manager.set("ENABLE_PROPOSER".to_string(), "false".to_string());
                    env_manager.save().expect("Failed to save .env file");
                    stn_log("Proposer flag set to disabled.");
                } else {
                    println!("No changes made to proposer configuration.");
                }
                return;
            }

            // Offer to restart the node to apply changes
            println!("Would you like to restart the node to apply changes? (y/n): ");
            let mut restart_input = String::new();
            io::stdin()
                .read_line(&mut restart_input)
                .expect("Failed to read input");
            if restart_input.trim() == "y" {
                restart(taiko_node_dir); // Call the restart function
            } else {
                println!("Changes will take effect after the next restart.");
            }
        }
        ConfigSubcommands::Zkp => {
            stn_log("ZKP setup coming soon.");
        }
        ConfigSubcommands::Sgx => {
            stn_log("Sgx setup coming soon.");
        }
    }
}

fn up(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }
    match docker::execute_docker_command(&["compose", "up", "-d"], taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn down(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }
    match docker::execute_docker_command(&["compose", "down"], taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn upgrade(taiko_node_dir: &Path) {
    // Check if Docker daemon is running
    if let Err(e) = docker::check_docker_daemon() {
        eprintln!("{}", e);
        return;
    }

    // Pull latest simple-taiko-node from GitHub
    let mut git_pull = Command::new("git")
        .current_dir(taiko_node_dir)
        .arg("pull")
        .arg("origin")
        .arg("main")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute git pull command.");

    let git_pull_status = git_pull
        .wait()
        .expect("Failed to wait for git pull to complete.");

    if git_pull_status.success() {
        utils::stn_log("Git pull successful.");
    } else {
        utils::stn_log("Git pull failed.");
    }

    // Pull latest docker images
    match docker::execute_docker_command(&["compose", "pull"], taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    // Update .env file with .env.sample using EnvManager
    let env_path = taiko_node_dir.join(".env");
    let sample_env_path = taiko_node_dir.join(".env.sample");
    let mut env_manager =
        EnvManager::new(&env_path).expect("Failed to initialize EnvManager for .env file");

    match env_manager.update_from_sample(&sample_env_path) {
        Ok(()) => utils::stn_log("Successfully updated .env file from .env.sample."),
        Err(e) => {
            eprintln!("Failed to update .env file from .env.sample: {}", e);
        }
    }
}

fn restart(taiko_node_dir: &Path) {
    down(taiko_node_dir);
    up(taiko_node_dir);
}

fn remove(taiko_node_dir: &Path) {
    // Check if taiko node is installed first
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }
    match docker::execute_docker_command(&["compose", "down", "-v"], taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    }

    utils::stn_log("simple-taiko-node volumes deleted from system");
    match fs::remove_dir_all(taiko_node_dir) {
        Ok(_) => {
            utils::stn_log("simple-taiko-node directory deleted from system");
        }
        Err(e) => {
            eprintln!("Failed to remove simple-taiko-node directory: {}", e);
        }
    }
}

fn logs(log_type: &LogsSubcommands, taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }

    let mut args = vec!["compose", "logs", "--tail=100", "-f"];

    match log_type {
        LogsSubcommands::All => {
            // Do nothing, no other args needed
        }
        LogsSubcommands::Execution => {
            args.push("l2_execution_engine");
        }
        LogsSubcommands::Driver => {
            args.push("taiko_client_driver");
        }
    }

    match docker::execute_docker_command(&args, taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

async fn status(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }

    let env_path = taiko_node_dir.join(".env");
    let env_manager = EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

    let l2_endpoint_port = env_manager
        .get("PORT_L2_EXECUTION_ENGINE_HTTP")
        .expect("PORT_L2_EXECUTION_ENGINE_HTTP not set");
    let l2_endpoint_http = format!("http://localhost:{}", l2_endpoint_port);

    match get_sync_state(&l2_endpoint_http).await {
        Ok(sync_state) => {
            if sync_state.is_syncing {
                println!("Syncing in progress: {:.2}% complete.", sync_state.progress);
            } else {
                println!("Node is not syncing.");
            }
        }
        Err(error) => {
            eprintln!("Error checking syncing status: {}", error);
        }
    }
}
