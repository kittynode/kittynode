use ethers::providers::{Http, Middleware, Provider, Ws};
use std::time::Duration;
use tokio::time::timeout;

use crate::constants;

pub async fn validate_endpoints(l1_endpoint_http: &str, l1_endpoint_ws: &str) -> (bool, bool) {
    let timeout_duration = Duration::new(constants::DEFAULT_NETWORK_TIMEOUT, 0);

    // Validate L1_ENDPOINT_HTTP
    let http_provider: Provider<Http> = match Provider::<Http>::try_from(l1_endpoint_http) {
        Ok(provider) => provider,
        Err(_) => {
            eprintln!("Failed to create HTTP provider. Please try another HTTP endpoint.");
            return (false, false);
        }
    };

    let http_result = timeout(timeout_duration, http_provider.get_block_number()).await;
    let http_validation = match http_result {
        Ok(Ok(_)) => true,
        _ => {
            eprintln!("Failed to fetch block number via HTTP or request timed out. Please try another HTTP endpoint.");
            false
        }
    };

    // Validate L1_ENDPOINT_WS
    let ws_provider: Provider<Ws> = match Provider::<Ws>::connect(l1_endpoint_ws).await {
        Ok(provider) => provider,
        Err(_) => {
            eprintln!("Failed to create WS provider. Please try another WS endpoint.");
            return (http_validation, false);
        }
    };

    let ws_result = timeout(timeout_duration, ws_provider.get_block_number()).await;
    let ws_validation = match ws_result {
        Ok(Ok(_)) => true,
        _ => {
            eprintln!("Failed to fetch block number via WS or request timed out. Please try another WS endpoint.");
            false
        }
    };

    (http_validation, ws_validation)
}
