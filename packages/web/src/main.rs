use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use kittynode_core::package::Package;

pub(crate) async fn root() -> &'static str {
    "Hello, World!"
}

pub(crate) async fn install_package(
    Path(name): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    kittynode_core::package::install_package(&name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

pub(crate) async fn delete_package(
    Path(name): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    kittynode_core::package::delete_package(&name, false)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::OK)
}

pub(crate) async fn get_installed_packages() -> Result<Json<Vec<Package>>, (StatusCode, String)> {
    kittynode_core::package::get_installed_packages()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/install_package/:name", post(install_package))
        .route("/delete_package/:name", post(delete_package))
        .route("/get_installed_packages", get(get_installed_packages));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
