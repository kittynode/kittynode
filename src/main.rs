use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
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
    Terminate,
    /// Handles logs operations
    Logs(Logs),
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

// Constant for simple-taiko-node repo url
const SIMPLE_TAIKO_NODE_REPO_URL: &str = "https://github.com/taikoxyz/simple-taiko-node";

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install => {
            install();
        }
        Commands::Config => {
            config();
        }
        Commands::Up => {
            up();
        }
        Commands::Down => {
            down();
        }
        Commands::Upgrade => {
            upgrade();
        }
        Commands::Terminate => {
            terminate();
        }
        Commands::Logs(logs) => {
            handle_docker_logs(&logs.subcommands);
        }
    }
}

fn handle_docker_logs(log_type: &LogsSubcommands) {
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

    execute_docker_command(&args);
}

fn install() {
    // Check we are inside an empty directory
    let cwd = env::current_dir().expect("Failed to get current directory");
    if !is_directory_empty(&cwd).expect("Failed to read the current directory") {
        stn_log("The current directory is not empty. Please run this in an empty directory.");
        return;
    }

    // Pull latest simple-taiko-node from GitHub
    let mut git_clone = Command::new("git")
        .arg("clone")
        .arg(SIMPLE_TAIKO_NODE_REPO_URL)
        .arg(".")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute git clone command.");

    let git_clone_status = git_clone
        .wait()
        .expect("Failed to wait for git clone to complete.");

    if git_clone_status.success() {
        stn_log("Git clone successful.");
    } else {
        stn_log("Git clone failed.");
    }

    // Copy .env.sample to .env
    let sample_path = Path::new(".env.sample");
    let env_path: &Path = Path::new(".env");
    std::fs::copy(sample_path, env_path).expect("Failed to copy .env.sample to .env");
}

fn config() {
    // Prompt for L1_ENDPOINT_HTTP and L1_ENDPOINT_WS
    let mut l1_endpoint_http = String::new();
    let mut l1_endpoint_ws = String::new();

    println!("Please enter your L1_ENDPOINT_HTTP:");
    std::io::stdin()
        .read_line(&mut l1_endpoint_http)
        .expect("Failed to read L1_ENDPOINT_HTTP");

    println!("Please enter your L1_ENDPOINT_WS:");
    std::io::stdin()
        .read_line(&mut l1_endpoint_ws)
        .expect("Failed to read L1_ENDPOINT_WS");

    l1_endpoint_http = l1_endpoint_http.trim().to_string();
    l1_endpoint_ws = l1_endpoint_ws.trim().to_string();

    // Update .env with L1_ENDPOINT_HTTP and L1_ENDPOINT_WS
    let env_path: &Path = Path::new(".env");
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

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(env_path)
        .expect("Failed to open .env file");
    file.write_all(new_contents.as_bytes())
        .expect("Failed to write to .env file");
}

fn up() {
    execute_docker_command(&["compose", "up", "-d"]);
    stn_log("simple-taiko-node successfully started");
}

fn down() {
    execute_docker_command(&["compose", "down"]);
    stn_log("simple-taiko-node successfully stopped");
}

fn upgrade() {
    // Check docker daemon
    if let Err(error_msg) = check_docker_daemon() {
        stn_log(&error_msg);
        return;
    }

    // Pull latest simple-taiko-node from GitHub
    let mut git_pull = Command::new("git")
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
        stn_log("Git pull successful.");
    } else {
        stn_log("Git pull failed.");
    }

    // Pull latest docker images
    execute_docker_command(&["compose", "pull"]);

    // Execute a script with bash: in ./scripts/util/update-env.sh
    let mut update_env = Command::new("bash")
        .arg("./script/util/update-env.sh")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute update-env.sh script.");

    let update_env_status = update_env
        .wait()
        .expect("Failed to wait for update-env.sh to complete.");

    if update_env_status.success() {
        stn_log("update-env.sh script successful.");
    } else {
        stn_log("update-env.sh script failed.");
    }
}

fn terminate() {
    execute_docker_command(&["compose", "down", "-v"]);
    stn_log("simple-taiko-node removed from system");
}

// Helper to execute docker commands
fn execute_docker_command(args: &[&str]) {
    // Check docker daemon
    if let Err(error_msg) = check_docker_daemon() {
        stn_log(&error_msg);
        return;
    }

    // Prepend sudo for linux
    let mut child = if cfg!(target_os = "linux") {
        Command::new("sudo")
            .args(["docker"].iter().chain(args))
            .spawn()
            .expect(&format!(
                "Failed to execute sudo docker command: {:?}",
                args
            ))
    } else {
        Command::new("docker")
            .args(args)
            .spawn()
            .expect(&format!("Failed to execute docker command: {:?}", args))
    };

    // Wait for the command to complete
    let result = child.wait();
    match result {
        Ok(status) if status.success() => {
            stn_log(&format!("Successfully executed command: {:?}", args));
        }
        _ => {
            stn_log(&format!(
                "Failed to wait for command to complete: {:?}",
                args
            ));
        }
    }
}

// Helper to check if Docker daemon is running and handle error logging
fn check_docker_daemon() -> Result<(), String> {
    let (command, args) = if cfg!(target_os = "linux") {
        ("sudo", vec!["docker", "version"])
    } else {
        ("docker", vec!["version"])
    };

    let docker_version = Command::new(command).args(args).output();

    match docker_version {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err("Docker daemon is not running. Please start Docker!".to_string()),
    }
}

// Helper to check if directory is empty
fn is_directory_empty(dir: &Path) -> io::Result<bool> {
    let mut entries = fs::read_dir(dir)?.peekable();
    Ok(entries.peek().is_none())
}

// Utility logging function for stn
fn stn_log(msg: &str) {
    println!("stn_log: {}", msg);
}
