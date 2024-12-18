use crate::domain::package::{Binding, Container};
use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions, StartContainerOptions},
    image::CreateImageOptions,
    network::{ConnectNetworkOptions, CreateNetworkOptions},
    secret::{ContainerSummary, HostConfig},
    Docker,
};
use eyre::{Report, Result};
use std::collections::HashMap;
use tokio_stream::StreamExt;
use tracing::{error, info};

pub(crate) fn get_docker_instance() -> Result<Docker> {
    // Try default connection
    if let Ok(docker) = Docker::connect_with_local_defaults() {
        return Ok(docker);
    }

    // Try rootless Docker socket on Linux
    #[cfg(target_os = "linux")]
    {
        let socket_path = format!("unix:///run/user/{}/docker.sock", users::get_current_uid());
        if let Ok(docker) =
            Docker::connect_with_unix(&socket_path, 120, bollard::API_DEFAULT_VERSION)
        {
            return Ok(docker);
        }
    }

    // If all attempts fail, return the original error
    Docker::connect_with_local_defaults().map_err(Report::from)
}

pub(crate) async fn create_or_recreate_network(docker: &Docker, network_name: &str) -> Result<()> {
    // Check if network already exists
    let network_exists = docker
        .list_networks::<String>(None)
        .await?
        .iter()
        .any(|n| n.name.as_deref() == Some(network_name));

    // Remove network if it already exists
    if network_exists {
        docker.remove_network(network_name).await?;
        info!("Removed existing network: '{}'", network_name);
    }

    // Create new network
    docker
        .create_network(CreateNetworkOptions {
            name: network_name,
            check_duplicate: true,
            driver: "bridge",
            ..Default::default()
        })
        .await?;
    info!("Created new network: '{}'", network_name);

    Ok(())
}

pub(crate) async fn find_container(docker: &Docker, name: &str) -> Result<Vec<ContainerSummary>> {
    let filters = HashMap::from([("name".to_string(), vec![name.to_string()])]);

    Ok(docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        }))
        .await?)
}

pub(crate) async fn remove_container(docker: &Docker, name: &str) -> Result<()> {
    for container in find_container(docker, name).await? {
        let id = container
            .id
            .ok_or_else(|| eyre::eyre!("Container ID was None"))?;
        docker.stop_container(&id, None).await.ok(); // Ignore stop errors
        docker.remove_container(&id, None).await?;
    }

    Ok(())
}

pub(crate) async fn pull_and_start_container(
    docker: &Docker,
    container: &Container,
    network_name: &str,
) -> Result<()> {
    let options = Some(CreateImageOptions {
        from_image: container.image.to_string(),
        tag: "latest".to_string(),
        ..Default::default()
    });

    let mut stream = docker.create_image(options, None, None);
    while let Some(item) = stream.next().await {
        match item {
            Ok(info) => info!("Pulling image info: {:?}", info),
            Err(e) => error!("Error pulling image: {:?}", e),
        }
    }

    let port_bindings = container
        .port_bindings
        .iter()
        .map(|(k, v)| (k.to_string(), Some(v.clone())))
        .collect();

    let bindings = container
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
        image: Some(container.image.to_string()),
        cmd: Some(container.cmd.clone()),
        host_config: Some(host_config),
        ..Default::default()
    };

    let created_container = docker
        .create_container(
            Some(CreateContainerOptions {
                name: container.name.to_string(),
                ..Default::default()
            }),
            config,
        )
        .await?;
    info!("Container {} created successfully.", container.name);

    docker
        .start_container(&created_container.id, None::<StartContainerOptions<String>>)
        .await?;
    info!("Container {} started successfully.", container.name);

    docker
        .connect_network(
            network_name,
            ConnectNetworkOptions {
                container: container.name.to_string(),
                endpoint_config: Default::default(),
            },
        )
        .await?;
    info!(
        "Container {} connected to network '{}'.",
        container.name, network_name
    );

    Ok(())
}

pub async fn get_container_logs(
    docker: &Docker,
    container_name: &str,
    tail_lines: Option<usize>,
) -> Result<Vec<String>> {
    let tail = tail_lines
        .map(|n| n.to_string())
        .unwrap_or_else(|| "all".to_string());

    let options = bollard::container::LogsOptions::<String> {
        stdout: true,
        stderr: true,
        follow: false,
        timestamps: true,
        tail,
        ..Default::default()
    };

    let mut stream = docker.logs(container_name, Some(options));
    let mut log_strings = Vec::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(output) => match output {
                bollard::container::LogOutput::StdOut { message }
                | bollard::container::LogOutput::StdErr { message } => {
                    log_strings.push(String::from_utf8_lossy(&message).to_string());
                }
                _ => {}
            },
            Err(e) => return Err(e.into()),
        }
    }

    Ok(log_strings)
}

fn create_binding_string(binding: &Binding) -> String {
    match &binding.options {
        Some(options) => format!("{}:{}:{}", binding.source, binding.destination, options),
        None => format!("{}:{}", binding.source, binding.destination),
    }
}
