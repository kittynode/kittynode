use bollard::Docker;
use eyre::Result;

pub async fn is_docker_running() -> Result<bool> {
    match Docker::connect_with_local_defaults() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
