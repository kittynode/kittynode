use clap::{Parser, Subcommand};
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
    /// Starts up simple-taiko-node in the background
    Up,
    /// Stops simple-taiko-node
    Down,
    /// Upgrades simple-taiko-node to the latest version
    Upgrade,
    /// Deletes simple-taiko-node instance
    Terminate,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
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
    }
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

// Utility logging function for stn
fn stn_log(msg: &str) {
    println!("stn_log: {}", msg);
}
