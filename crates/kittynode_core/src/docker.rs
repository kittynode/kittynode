use bollard::{network::CreateNetworkOptions, Docker};
use eyre::{Context, Result};
use tracing::info;

pub fn is_docker_running() -> Result<bool> {
    match get_docker_instance() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub(crate) fn get_docker_instance() -> Result<Docker> {
    Docker::connect_with_local_defaults().wrap_err("Failed to connect to Docker")
}

pub(crate) async fn create_or_recreate_network(docker: &Docker, network_name: &str) -> Result<()> {
    if let Ok(networks) = docker.list_networks::<String>(None).await {
        if networks
            .iter()
            .any(|n| n.name == Some(network_name.to_string()))
        {
            docker
                .remove_network(network_name)
                .await
                .wrap_err("Failed to remove existing network")?;
            info!("Removed existing network: {}", network_name);
        }
    }

    let network_options = CreateNetworkOptions {
        name: network_name,
        check_duplicate: true,
        driver: "bridge",
        ..Default::default()
    };

    docker
        .create_network(network_options)
        .await
        .wrap_err("Failed to create network")?;

    info!("Created network: {}", network_name);
    Ok(())
}
