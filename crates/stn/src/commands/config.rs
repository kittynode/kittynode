use crate::commands::restart;
use crate::{constants, env_manager::EnvManager, network, ConfigSubcommands};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Password, Select};
use std::path::Path;

// Main entry point for the config command
pub async fn config(config_subcommand: &ConfigSubcommands, taiko_node_dir: &Path) {
    let env_path = taiko_node_dir.join(".env");
    let mut env_manager = EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

    match config_subcommand {
        ConfigSubcommands::Full => {
            handle_full_config(&mut env_manager).await;
        }
        ConfigSubcommands::Proposer => {
            handle_proposer_config(&mut env_manager, taiko_node_dir).await;
        }
        ConfigSubcommands::Zkp => {
            println!("ZKP setup coming soon.");
        }
        ConfigSubcommands::Sgx => {
            println!("SGX setup coming soon.");
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

// Handle proposer configuration
async fn handle_proposer_config(env_manager: &mut EnvManager, taiko_node_dir: &Path) {
    let current_state = env_manager
        .get("ENABLE_PROPOSER")
        .expect("ENABLE_PROPOSER key is missing or unreadable. Please check your .env file.");

    // Proposer is disabled
    if current_state != "true" {
        // User wants to enable it
        if confirm_with_message(
            "The node is currently not configured as a proposer. Would you like to enable it?",
        ) {
            if init_or_update_proposer(env_manager).await {
                offer_restart(taiko_node_dir).await;
            } else {
                println!("Failed to initialize or update proposer configuration.");
                println!("No changes made to proposer configuration.");
            }
        }
        // User says no
        else {
            println!("No changes made to proposer configuration.");
        }
    }
    // Proposer is enabled
    else if current_state == "true" {
        handle_existing_proposer_configuration(env_manager, taiko_node_dir).await;
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

// Offer to restart the node
async fn offer_restart(taiko_node_dir: &Path) {
    if confirm_with_message("Would you like to restart the node to apply changes?") {
        restart(taiko_node_dir).await;
    } else {
        println!("Changes will take effect after the next restart.");
    }
}

// Setup and enable proposer configuration. This is used for update or initial setup.
// Returns true if successful, false otherwise
async fn init_or_update_proposer(env_manager: &mut EnvManager) -> bool {
    // Verify that the node is synced
    if !validate_node_sync_status(env_manager).await {
        return false;
    }

    // Set prover endpoint
    let prover_endpoint = select_prover_endpoint().await;
    if !network::is_prover_api_functional(&prover_endpoint).await {
        println!("The provided prover endpoint is not functional.");
        return false;
    }

    // Set private key
    let l1_proposer_private_key = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Please enter your L1_PROPOSER_PRIVATE_KEY (use a test account only!)")
        .interact()
        .expect("Failed to read L1_PROPOSER_PRIVATE_KEY");
    if l1_proposer_private_key.starts_with("0x") {
        println!("Private key should not start with 0x");
        return false;
    }

    // Save the validated data
    env_manager.set("ENABLE_PROPOSER".to_string(), "true".to_string());
    env_manager.set(
        "L1_PROPOSER_PRIVATE_KEY".to_string(),
        l1_proposer_private_key,
    );
    env_manager.set("PROVER_ENDPOINTS".to_string(), prover_endpoint);
    env_manager.save().expect("Failed to save .env file");

    true
}

// Handle configuration when proposer is already enabled
async fn handle_existing_proposer_configuration(
    env_manager: &mut EnvManager,
    taiko_node_dir: &Path,
) {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("The node is already configured as a proposer. What would you like to do?")
        .default(0)
        .items(&["Disable", "Update", "Cancel"])
        .interact()
        .unwrap();

    match selection {
        0 => {
            env_manager.set("ENABLE_PROPOSER".to_string(), "false".to_string());
            env_manager.save().expect("Failed to save .env file");
            println!("Proposer flag set to disabled.");
            offer_restart(taiko_node_dir).await;
        }
        1 => {
            if init_or_update_proposer(env_manager).await {
                offer_restart(taiko_node_dir).await;
            } else {
                println!("Failed to update proposer configuration.");
                println!("No changes made to proposer configuration.");
            }
        }
        2 => println!("No changes made to proposer configuration."),
        _ => unreachable!("Invalid selection"),
    }
}

async fn validate_node_sync_status(env_manager: &EnvManager) -> bool {
    let local_http = format!(
        "http://localhost:{}",
        env_manager
            .get("PORT_L2_EXECUTION_ENGINE_HTTP")
            .expect("PORT_L2_EXECUTION_ENGINE_HTTP not set")
    );
    let canonical_http = constants::KATLA_RPC_URL.to_string();
    if network::is_synced(&local_http, &canonical_http).await {
        true
    } else {
        println!("Node is not detected or fully synced. Make sure you have a functional full node first!");
        false
    }
}

async fn select_prover_endpoint() -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a prover endpoint configuration:")
        .default(0)
        .items(&[
            "Enter a prover endpoint",
            "Use the marketplace prover endpoint",
        ])
        .interact()
        .unwrap();

    match selection {
        0 => Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Please enter your desired prover endpoint")
            .interact_text()
            .expect("Failed to read the desired prover endpoint")
            .trim()
            .to_string(),
        1 => constants::DEFAULT_PROVER_URL.to_string(),
        _ => unreachable!("Invalid selection"),
    }
}
