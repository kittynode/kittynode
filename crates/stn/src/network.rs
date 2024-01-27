use ethers::{
    providers::{Http, Middleware, Provider, Ws},
    types::SyncingStatus,
};
use std::{fmt, time::Duration};
use tokio::time::timeout;

use crate::constants;

/// Custom error to handle different cases in is_syncing function.
pub enum SyncError {
    ProviderCreationFailed,
    SyncStatusFetchFailed,
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyncError::ProviderCreationFailed => write!(f, "Failed to create HTTP provider."),
            SyncError::SyncStatusFetchFailed => write!(f, "Failed to retrieve syncing status."),
        }
    }
}

/// Returns the sync status of the L1 endpoint. If syncing, returns progress as percentage.
pub async fn is_syncing(l1_endpoint_http: &str) -> Result<f64, SyncError> {
    let http_provider: Provider<Http> = Provider::<Http>::try_from(l1_endpoint_http)
        .map_err(|_| SyncError::ProviderCreationFailed)?;

    match http_provider.syncing().await {
        Ok(SyncingStatus::IsSyncing(sync_progress)) => {
            // print out debug data, TODO: don't print out all this debug stuff
            println!(
                "Syncing: current_block: {}, highest_block: {}",
                sync_progress.current_block, sync_progress.highest_block
            );
            let progress = if sync_progress.highest_block > sync_progress.current_block {
                100.0 * (sync_progress.current_block.as_u64() as f64)
                    / (sync_progress.highest_block.as_u64() as f64)
            } else {
                100.0 // Assuming that if current_block is not less than highest_block, syncing is complete.
            };
            Ok(progress)
        }
        Ok(SyncingStatus::IsFalse) => {
            // print out debug log
            println!("Syncing: false");
            return Ok(0.0);
        }
        Err(_) => Err(SyncError::SyncStatusFetchFailed),
    }
}

/// Validates the endpoints and returns a tuple of booleans indicating whether the endpoints are valid.
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

    let http_result = timeout(timeout_duration, http_provider.get_block(1)).await;
    let http_validation = match http_result {
        Ok(Ok(_)) => true,
        _ => {
            eprintln!("Failed to fetch block number 1 via HTTP or request timed out. Please try another HTTP endpoint and make sure it is an archive node!");
            false
        }
    };

    // Validate L1_ENDPOINT_WS
    let ws_provider = match connect_ws_with_timeout(l1_endpoint_ws, timeout_duration).await {
        Ok(provider) => provider,
        Err(error_msg) => {
            eprintln!("{}", error_msg);
            return (http_validation, false);
        }
    };

    let ws_result = timeout(timeout_duration, ws_provider.get_block(1)).await;
    let ws_validation = match ws_result {
        Ok(Ok(_)) => true,
        _ => {
            eprintln!("Failed to fetch block number 1 via WS or request timed out. Please try another WS endpoint and make sure it is an archive node!");
            false
        }
    };

    (http_validation, ws_validation)
}

// Helper to timeout the ws connection because invalid ws endpoints can hang for a long time
// and didn't find another util for it in ethers. Example: ws://192.168
async fn connect_ws_with_timeout(
    ws_endpoint: &str,
    timeout_duration: Duration,
) -> Result<Provider<Ws>, &'static str> {
    match timeout(timeout_duration, Provider::<Ws>::connect(ws_endpoint)).await {
        Ok(Ok(provider)) => Ok(provider),
        Ok(Err(_)) => Err("Failed to create WS provider."),
        Err(_) => Err("WS connection attempt timed out."),
    }
}
