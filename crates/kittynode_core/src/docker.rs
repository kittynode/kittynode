use bollard::{network::CreateNetworkOptions, Docker};
use eyre::{Context, Result};
use tracing::info;

fn get_docker_instance() -> Result<Docker> {
    Docker::connect_with_local_defaults().wrap_err("Failed to connect to Docker")
}

pub fn is_docker_running() -> Result<bool> {
    match get_docker_instance() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

async fn create_network(docker: &Docker, network_name: &str) -> Result<String> {
    let network_options = CreateNetworkOptions {
        name: network_name,
        check_duplicate: true,
        driver: "bridge",
        ..Default::default()
    };

    let network_id = docker
        .create_network(network_options)
        .await?
        .id
        .ok_or_else(|| eyre::eyre!("Network creation succeeded but no ID was returned"))?;

    info!("Docker network created: {}", network_id);

    Ok(network_id)
}
