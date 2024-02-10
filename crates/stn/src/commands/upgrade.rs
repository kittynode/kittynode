use dialoguer::{theme::ColorfulTheme, Confirm};
use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::{commands::restart, docker, env_manager::EnvManager};

pub async fn upgrade(taiko_node_dir: &Path) {
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
        println!("Git pull successful.");
    } else {
        println!("Git pull failed.");
    }

    // Pull latest docker images
    match docker::execute_docker_command(&["compose", "pull"], taiko_node_dir) {
        Ok(msg) => {
            println!("{}", msg);
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
        Ok(()) => println!("Successfully updated .env file from .env.sample."),
        Err(e) => {
            eprintln!("Failed to update .env file from .env.sample: {}", e);
        }
    }

    // Node has been updated, ask if they'd like to restart
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to restart the node to apply changes?")
        .interact()
        .expect("Failed to read input")
    {
        restart(taiko_node_dir).await;
    } else {
        println!("Changes will take effect after the next restart.");
    }
}
