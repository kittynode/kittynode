use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};

pub(crate) async fn root() -> &'static str {
    "Hello, World!"
}

pub(crate) async fn install_package(Path(name): Path<String>) -> Result<(), String> {
    kittynode_core::package::install_package(&name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub(crate) async fn delete_package(Path(name): Path<String>) -> Result<(), String> {
    kittynode_core::package::delete_package(&name, false)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/install_package/:name", post(install_package))
        .route("/delete_package/:name", post(delete_package));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
