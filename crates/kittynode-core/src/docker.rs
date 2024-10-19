use crate::package::{create_binding_string, Container};
use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions, StartContainerOptions},
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
        match connection.version().await {
            Ok(_) => Ok(true),   // Version check successful, Docker is running
            Err(_) => Ok(false), // Version check failed, Docker is not running
        }
    } else {
        Ok(false) // Docker connection failed
    }
}

pub async fn find_container(docker: &Docker, name: &str) -> Result<Vec<ContainerSummary>> {
    let filters = HashMap::from([("name".to_string(), vec![name.to_string()])]);
    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        }))
        .await
        .wrap_err("Failed to list containers")?;

    Ok(containers)
}

pub async fn remove_container(docker: &Docker, name: &str) -> Result<()> {
    for container in find_container(docker, name).await? {
        let id = container
            .id
            .ok_or_else(|| eyre::eyre!("Container ID was None"))?;
        docker.stop_container(&id, None).await.ok(); // Ignore stop errors
        docker.remove_container(&id, None).await?;
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
