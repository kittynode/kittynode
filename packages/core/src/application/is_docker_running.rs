use crate::infra::docker::get_docker_instance;

pub async fn is_docker_running() -> bool {
    match get_docker_instance() {
        Ok(connection) => connection.version().await.is_ok(),
        _ => {
            false // Docker connection failed
        }
    }
}
