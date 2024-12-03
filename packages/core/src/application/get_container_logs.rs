use crate::infra::docker::get_docker_instance;
use eyre::Result;
use tokio_stream::StreamExt;

pub async fn get_container_logs(container_name: &str) -> Result<Vec<String>> {
    let docker = get_docker_instance()?;

    let options = bollard::container::LogsOptions::<String> {
        stdout: true,
        stderr: true,
        follow: false, // We're using polling instead of streaming
        timestamps: true,
        tail: "100".to_string(),
        ..Default::default()
    };

    let logs = docker.logs(container_name, Some(options));
    let mut log_strings = Vec::new();

    let mut stream = logs;

    while let Some(result) = stream.next().await {
        match result {
            Ok(output) => match output {
                bollard::container::LogOutput::StdOut { message } => {
                    log_strings.push(String::from_utf8_lossy(&message).to_string());
                }
                bollard::container::LogOutput::StdErr { message } => {
                    log_strings.push(String::from_utf8_lossy(&message).to_string());
                }
                _ => {}
            },
            Err(e) => return Err(e.into()),
        }
    }

    Ok(log_strings)
}
