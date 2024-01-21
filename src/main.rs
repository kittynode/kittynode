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
    Up,
    Down,
    Terminate,
    Upgrade,
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
        Commands::Terminate => {
            terminate();
        }
        Commands::Upgrade => {
            upgrade();
        }
    }
}

fn up() {
    println!("up");
}

fn down() {
    println!("down");
}

fn terminate() {
    println!("terminate");
}

fn upgrade() {
    // Check if Docker daemon is running
    let docker_daemon_running = check_docker_daemon();

    if !docker_daemon_running {
        stn_log("Docker daemon is not running. Please start Docker!");
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

    // Update docker images
    let mut docker_compose_pull = Command::new("docker-compose")
        .arg("pull")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute docker-compose pull command.");

    let docker_compose_pull_status = docker_compose_pull
        .wait()
        .expect("Failed to wait for docker-compose pull to complete.");

    if docker_compose_pull_status.success() {
        stn_log("Docker compose pull successful.");
    } else {
        stn_log("Docker compose pull failed.");
    }

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

fn check_docker_daemon() -> bool {
    let docker_version = Command::new("docker")
        .arg("version")
        .output()
        .expect("Failed to execute docker version command.");

    docker_version.status.success()
}

fn stn_log(msg: &str) {
    println!("stn_log: {}", msg);
}
