use crate::infra::docker::get_docker_instance;

pub async fn is_docker_running() -> bool {
    if let Ok(connection) = get_docker_instance() {
        connection.version().await.is_ok()
    } else {
        false // Docker connection failed
    }
}
