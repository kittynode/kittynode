use std::{path::Path, process::Command};

#[derive(Debug)]
pub enum DockerError {
    DockerNotRunning,
    CommandFailed(Vec<String>),
}

impl std::fmt::Display for DockerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DockerError::DockerNotRunning => {
                write!(
                    f,
                    "Docker daemon is not running. Please start Docker or try restarting it!"
                )
            }
            DockerError::CommandFailed(ref args) => write!(f, "Command failed: {:?}", args),
        }
    }
}

/// Execute a docker command with the corrects args (prepending sudo for linux) and working directory
pub fn execute_docker_command(args: &[&str], working_dir: &Path) -> Result<String, DockerError> {
    check_docker_daemon()?;

    // Prepend sudo for linux
    let mut child = if cfg!(target_os = "linux") {
        Command::new("sudo")
            .current_dir(working_dir)
            .args(["docker"].iter().chain(args))
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute sudo docker command: {:?}", args))
    } else {
        Command::new("docker")
            .current_dir(working_dir)
            .args(args)
            .spawn()
            .unwrap_or_else(|_| panic!("Failed to execute docker command: {:?}", args))
    };

    let result = child.wait();
    match result {
        Ok(status) if status.success() => {
            let success_message = format!("Successfully executed command: {:?}", args);
            Ok(success_message)
        }
        _ => {
            let args_vec: Vec<String> = args.iter().map(|&arg| arg.to_string()).collect();
            Err(DockerError::CommandFailed(args_vec))
        }
    }
}

/// Check if Docker daemon is running
pub fn check_docker_daemon() -> Result<(), DockerError> {
    let (command, args) = if cfg!(target_os = "linux") {
        ("sudo", vec!["docker", "version"])
    } else {
        ("docker", vec!["version"])
    };

    let docker_version = Command::new(command).args(args).output();

    match docker_version {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(DockerError::DockerNotRunning),
    }
}
