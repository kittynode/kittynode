use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub(crate) async fn root() -> &'static str {
    "Hello, World!"
}

pub(crate) async fn system_info() -> impl IntoResponse {
    match kittynode_core::system_info::get_system_info() {
        Ok(system_info) => (StatusCode::OK, Json(system_info)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Error retrieving system info: {}", e),
            }),
        )
            .into_response(),
    }
}
