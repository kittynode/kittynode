mod constants;
mod docker;
mod utils;

use clap::{Parser, Subcommand};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a simple-taiko-node instance
    Install,
    /// Configs a simple-taiko-node instance
    Config,
    /// Starts up simple-taiko-node in the background
    Up,
    /// Stops simple-taiko-node
    Down,
    /// Upgrades simple-taiko-node to the latest version
    Upgrade,
    /// Deletes simple-taiko-node instance
    Remove,
    /// Handles logs operations
    Logs(Logs),
    /// Status of Taiko nodes
    Status,
}

#[derive(Parser)]
struct Logs {
    #[command(subcommand)]
    subcommands: LogsSubcommands,
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

fn main() {
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
        Commands::Config => {
            config(&taiko_node_dir);
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
        Commands::Remove => {
            remove(&taiko_node_dir);
        }
        Commands::Logs(logs_subcommands) => {
            logs(&logs_subcommands.subcommands, &taiko_node_dir);
        }
        Commands::Status => {
            status(&taiko_node_dir);
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

fn config(taiko_node_dir: &Path) {
    let mut l1_endpoint_http = String::new();
    let mut l1_endpoint_ws = String::new();

    print!("Please enter your L1_ENDPOINT_HTTP: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut l1_endpoint_http)
        .expect("Failed to read L1_ENDPOINT_HTTP");

    print!("Please enter your L1_ENDPOINT_WS: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut l1_endpoint_ws)
        .expect("Failed to read L1_ENDPOINT_WS");

    l1_endpoint_http = l1_endpoint_http.trim().to_string();
    l1_endpoint_ws = l1_endpoint_ws.trim().to_string();

    // Update .env with L1_ENDPOINT_HTTP and L1_ENDPOINT_WS
    let env_path = Path::new(&taiko_node_dir).join(".env");
    let mut file = File::open(env_path).expect("Failed to open .env file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read .env file");

    let new_contents = contents
        .lines()
        .map(|line| {
            if line.starts_with("L1_ENDPOINT_HTTP") {
                return format!("L1_ENDPOINT_HTTP={}", l1_endpoint_http);
            } else if line.starts_with("L1_ENDPOINT_WS") {
                return format!("L1_ENDPOINT_WS={}", l1_endpoint_ws);
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let env_path = Path::new(&taiko_node_dir).join(".env");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(env_path)
        .expect("Failed to open .env file");
    file.write_all(new_contents.as_bytes())
        .expect("Failed to write to .env file");
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

    // Execute a script with bash: in ./scripts/util/update-env.sh
    let mut update_env = Command::new("bash")
        .current_dir(taiko_node_dir)
        .arg("./script/util/update-env.sh")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute update-env.sh script.");

    let update_env_status = update_env
        .wait()
        .expect("Failed to wait for update-env.sh to complete.");

    if update_env_status.success() {
        utils::stn_log("update-env.sh script successful.");
    } else {
        utils::stn_log("update-env.sh script failed.");
    }
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

    let mut args = vec!["compose", "logs", "-f"];

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

fn status(taiko_node_dir: &Path) {
    // Check taiko node is installed first
    if !taiko_node_dir.exists() {
        utils::stn_log("simple-taiko-node is not installed.");
        return;
    }

    match docker::execute_docker_command(&["ps"], taiko_node_dir) {
        Ok(msg) => {
            utils::stn_log(&msg);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
