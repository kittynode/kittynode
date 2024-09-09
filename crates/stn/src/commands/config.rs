use crate::{env_manager::EnvManager, network, ConfigSubcommands};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::path::Path;

// Main entry point for the config command
pub async fn config(config_subcommand: &ConfigSubcommands, taiko_node_dir: &Path) {
    let env_path = taiko_node_dir.join(".env");
    let mut env_manager = EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

    match config_subcommand {
        ConfigSubcommands::Full => {
            handle_full_config(&mut env_manager).await;
        }
    }
}

// Handle full configuration
async fn handle_full_config(env_manager: &mut EnvManager) {
    if confirm_with_message("Do you have an HTTP and WS endpoint for a Holesky L1 archive node?") {
        if prompt_validate_save_endpoints(env_manager).await {
            env_manager.save().expect("Failed to save .env file");
            println!("simple-taiko-node successfully configured.");
        }
    } else {
        println!(
            concat!("\nYou must have an HTTP and WS endpoint for a Holesky L1 archive node to configure a Taiko node. You can either:\n",
                    "  1. Install a Holesky L1 archive node and run it locally\n",
                    "  2. Use a public Holesky L1 archive node from an RPC provider (will have to pay or eventually get rate limited)\n",
                    "\nSee the docs at https://docs.taiko.xyz/guides/run-a-taiko-node for more info.")
        );
    }
}

// Confirm user input with a custom message
fn confirm_with_message(message: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(message)
        .interact()
        .expect("Failed to read input")
}

// Prompt for endpoints, validate them, and save to env_manager if valid
async fn prompt_validate_save_endpoints(env_manager: &mut EnvManager) -> bool {
    let l1_endpoint_http = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter your L1_ENDPOINT_HTTP")
        .interact_text()
        .expect("Failed to read L1_ENDPOINT_HTTP")
        .trim()
        .to_string();

    let l1_endpoint_ws = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter your L1_ENDPOINT_WS")
        .interact_text()
        .expect("Failed to read L1_ENDPOINT_WS")
        .trim()
        .to_string();

    let (http_valid, ws_valid) =
        network::validate_endpoints(&l1_endpoint_http, &l1_endpoint_ws).await;

    if http_valid && ws_valid {
        env_manager.set("L1_ENDPOINT_HTTP".to_string(), l1_endpoint_http);
        env_manager.set("L1_ENDPOINT_WS".to_string(), l1_endpoint_ws);
        true
    } else {
        println!("One or both of the endpoints are invalid.");
        false
    }
}
