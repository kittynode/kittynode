use std::{path::Path, process::Command};

/// Execute a docker command with the corrects args (prepending sudo for linux) and working directory
pub fn execute_docker_command(args: &[&str], working_dir: &Path) -> Result<(), String> {
    // Check docker daemon
    check_docker_daemon()?;

    // Prepend sudo for linux
    let mut child = if cfg!(target_os = "linux") {
        Command::new("sudo")
            .current_dir(working_dir)
            .args(["docker"].iter().chain(args))
            .spawn()
            .expect(&format!(
                "Failed to execute sudo docker command: {:?}",
                args
            ))
    } else {
        Command::new("docker")
            .current_dir(working_dir)
            .args(args)
            .spawn()
            .expect(&format!("Failed to execute docker command: {:?}", args))
    };

    let result = child.wait();
    match result {
        Ok(status) if status.success() => Ok(()),
        _ => Err(format!("Failed to execute command: {:?}", args)),
    }
}

/// Check if Docker daemon is running
pub fn check_docker_daemon() -> Result<(), String> {
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
