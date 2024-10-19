use crate::package::{create_binding_string, Container};
use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    image::CreateImageOptions,
    network::{ConnectNetworkOptions, CreateNetworkOptions},
    secret::{ContainerSummary, HostConfig, PortBinding},
    Docker,
};
use eyre::{Context, Result};
use std::collections::HashMap;
use tokio_stream::StreamExt;
use tracing::{error, info};

pub(crate) fn get_docker_instance() -> Result<Docker> {
    Docker::connect_with_local_defaults().wrap_err("Failed to connect to Docker")
}

pub(crate) async fn create_or_recreate_network(docker: &Docker, network_name: &str) -> Result<()> {
    let networks = docker
        .list_networks::<String>(None)
        .await
        .wrap_err("Failed to list Docker networks")?;

    if networks
        .iter()
        .any(|n| n.name.as_deref() == Some(network_name))
    {
        info!("Removing existing network: {}", network_name);
        docker
            .remove_network(network_name)
            .await
            .wrap_err("Failed to remove existing network")?;
    }

    let options = CreateNetworkOptions {
        name: network_name,
        check_duplicate: true,
        driver: "bridge",
        ..Default::default()
    };

    docker
        .create_network(options)
        .await
        .wrap_err("Failed to create network")?;
    info!("Created network: {}", network_name);
    Ok(())
}

pub async fn is_docker_running() -> Result<bool> {
    if let Ok(connection) = get_docker_instance() {
        match connection.ping().await {
            Ok(_) => Ok(true),   // Ping successful, Docker is running
            Err(_) => Ok(false), // Ping failed, Docker is not running
        }
    } else {
        Ok(false) // Docker connection failed
    }
}

pub async fn find_container(docker: &Docker, name: &str) -> Result<Option<ContainerSummary>> {
    let containers = docker
        .list_containers::<String>(None)
        .await
        .wrap_err("Failed to list Docker containers")?;

    Ok(containers.into_iter().find(|c| {
        c.names.as_ref().map_or(false, |names| {
            names.iter().any(|n| n.as_str() == format!("/{}", name))
        })
    }))
}

pub async fn remove_container(docker: &Docker, container_name: &str) -> Result<()> {
    if let Some(container) = find_container(docker, container_name).await? {
        if let Some(container_id) = container.id.as_deref() {
            info!("Stopping container: {}", container_name);
            docker.stop_container(container_id, None).await.ok(); // Ignore stop errors
            docker
                .remove_container(container_id, None)
                .await
                .wrap_err("Failed to remove container")?;
            info!("Removed container: {}", container_name);
        } else {
            tracing::warn!("Container ID for '{}' was None", container_name);
        }
    }
    Ok(())
}

pub async fn pull_and_start_container(
    docker: &Docker,
    container: &Container,
    network_name: &str,
) -> Result<()> {
    let options = Some(CreateImageOptions {
        from_image: container.image,
        tag: "latest",
        ..Default::default()
    });

    let mut stream = docker.create_image(options, None, None);
    while let Some(item) = stream.next().await {
        match item {
            Ok(info) => info!("Pulling image info: {:?}", info),
            Err(e) => error!("Error pulling image: {:?}", e),
        }
    }

    let port_bindings: HashMap<String, Option<Vec<PortBinding>>> = container
        .port_bindings
        .iter()
        .map(|(k, v)| (k.to_string(), Some(v.clone())))
        .collect();

    let bindings: Vec<String> = container
        .volume_bindings
        .iter()
        .chain(&container.file_bindings)
        .map(create_binding_string)
        .collect();

    let host_config = HostConfig {
        binds: Some(bindings),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };

    let config = Config {
        image: Some(container.image),
        cmd: Some(container.cmd.clone()),
        host_config: Some(host_config),
        ..Default::default()
    };

    let created_container = docker
        .create_container(
            Some(CreateContainerOptions {
                name: container.name,
                ..Default::default()
            }),
            config,
        )
        .await
        .wrap_err("Failed to create container")?;

    docker
        .start_container(&created_container.id, None::<StartContainerOptions<String>>)
        .await
        .wrap_err("Failed to start container")?;

    info!("Container {} started successfully.", container.name);

    docker
        .connect_network(
            network_name,
            ConnectNetworkOptions {
                container: container.name,
                endpoint_config: Default::default(),
            },
        )
        .await
        .wrap_err("Failed to connect container to network")?;

    Ok(())
}
