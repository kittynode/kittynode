use std::io;
use std::io::Write;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

use crate::commands::restart;
use crate::constants;
use crate::env_manager::EnvManager;
use crate::network;
use crate::utils::stn_log;
use crate::ConfigSubcommands;
use crate::Path;

pub async fn config(config_subcommand: &ConfigSubcommands, taiko_node_dir: &Path) {
    match config_subcommand {
        ConfigSubcommands::Full => {
            let has_endpoints = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you have an HTTP and WS endpoint for a Holesky L1 archive node?")
                .interact()
                .expect("Failed to read input");

            if !has_endpoints {
                println!(
                        concat!("\nYou must have an HTTP and WS endpoint for a Holesky L1 archive node to configure a Taiko node. You can either:\n",
                                "  1. Install a Holesky L1 archive node and run it locally\n",
                                "  2. Use a public Holesky L1 archive node from an RPC provider (will have to pay or eventually get rate limited)\n",
                                "\nSee the docs at https://docs.taiko.xyz/guides/run-a-taiko-node for more info."));
                return;
            }

            let l1_endpoint_http: String = dialoguer::Input::<String>::new()
                .with_prompt("Please enter your L1_ENDPOINT_HTTP")
                .interact_text()
                .expect("Failed to read L1_ENDPOINT_HTTP")
                .trim()
                .to_string();

            let l1_endpoint_ws: String = dialoguer::Input::<String>::new()
                .with_prompt("Please enter your L1_ENDPOINT_WS")
                .interact_text()
                .expect("Failed to read L1_ENDPOINT_WS")
                .trim()
                .to_string();

            let (http_valid, ws_valid) =
                network::validate_endpoints(&l1_endpoint_http, &l1_endpoint_ws).await;

            if http_valid && ws_valid {
                println!("Both HTTP and WS endpoints are valid.");
            } else {
                println!("One or both of the endpoints are invalid.");
                return; // Don't continue if endpoints are invalid
            }

            // Initialize EnvManager and update .env with L1_ENDPOINT_HTTP and L1_ENDPOINT_WS
            let env_path = taiko_node_dir.join(".env");
            let mut env_manager =
                EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

            env_manager.set("L1_ENDPOINT_HTTP".to_string(), l1_endpoint_http);
            env_manager.set("L1_ENDPOINT_WS".to_string(), l1_endpoint_ws);
            env_manager.save().expect("Failed to save .env file");

            println!("simple-taiko-node successfully configured.");
        }
        ConfigSubcommands::Proposer => {
            // Initialize EnvManager and var to track intent
            let env_path = taiko_node_dir.join(".env");
            let mut env_manager =
                EnvManager::new(&env_path).expect("Failed to initialize EnvManager");

            // Check current state of the proposer
            let current_state = env_manager
                .get("ENABLE_PROPOSER")
                .unwrap_or("false".to_string());

            // Proposer is disabled
            if current_state != "true" {
                let enable_node_as_proposer = dialoguer::Confirm::new()
                    .with_prompt("The node is currently not configured as a proposer. Would you like to enable it?")
                    .default(false)
                    .interact()
                    .expect("Failed to get user response");

                // Enable proposer
                if enable_node_as_proposer {
                    // Check if they have a node installed, running, and fully synced
                    let local_http = format!(
                        "http://localhost:{}",
                        env_manager
                            .get("PORT_L2_EXECUTION_ENGINE_HTTP")
                            .expect("PORT_L2_EXECUTION_ENGINE_HTTP not set")
                    );
                    let canonical_http = constants::KATLA_RPC_URL.to_string();
                    let node_synced = network::is_synced(&local_http, &canonical_http).await;
                    if !node_synced {
                        println!("Node is not installed, running, or fully synced.");
                        return;
                    }
                    // Check if they have a local prover running
                    let local_prover_running = env_manager
                        .get("PROVER_ENDPOINTS")
                        .expect("PROVER_ENDPOINTS not set.")
                        .to_string()
                        .contains("taiko_client_prover_relayer");
                    let mut is_local_prover_functional = false;
                    // Local prover set
                    if local_prover_running {
                        is_local_prover_functional =
                            network::is_prover_api_functional(&local_http).await;
                        if !is_local_prover_functional {
                            println!("Local prover is running but the API is not functional.");
                        }
                    }
                    // If local prover running but not functional
                    if local_prover_running && !is_local_prover_functional {
                        // If they don't have a local prover running, ask if they would like to setup a marketplace prover
                        let setup_marketplace_prover = dialoguer::Confirm::new()
                            .with_prompt("Would you like to set up a marketplace prover?")
                            .default(false)
                            .interact()
                            .expect("Failed to get user response");
                        if setup_marketplace_prover {
                            // Healthcheck the marketplace prover
                            if !network::is_prover_api_functional(constants::DEFAULT_PROVER_URL)
                                .await
                            {
                                // If the marketplace prover fails, send them to the docs to find another marketplace prover
                                println!("Marketplace prover failed. Please consult the documentation to find another marketplace prover.");
                                return;
                            } else {
                                // If the marketplace prover succeeds, set the variable in their .env with env_manager
                                env_manager.set(
                                    "PROVER_ENDPOINTS".to_string(),
                                    constants::DEFAULT_PROVER_URL.to_string(),
                                );
                                stn_log(&format!(
                                    "Set {} as marketplace prover.",
                                    constants::DEFAULT_PROVER_URL
                                ));
                            }
                        } else {
                            println!("No changes made to proposer configuration.");
                            return;
                        }
                    }
                    // Marketplace prover running but not functional
                    if !local_prover_running
                        && !network::is_prover_api_functional(constants::DEFAULT_PROVER_URL).await
                    {
                        println!("Marketplace prover failed. Please consult the documentation to find another marketplace prover. No changes made to proposer configuration.");
                        return;
                    }
                    // Now prompt and set their proposer private key
                    let l1_proposer_private_key: String = dialoguer::Password::new()
                        .with_prompt(
                            "Please enter your L1_PROPOSER_PRIVATE_KEY (use a test account only!)",
                        )
                        .interact()
                        .expect("Failed to read L1_PROPOSER_PRIVATE_KEY");

                    // Now that their prover is set, and functional, and we have verified they are fully synced, proceed to enable_proposer flag to true, and set private key
                    env_manager.set("ENABLE_PROPOSER".to_string(), "true".to_string());
                    env_manager.set(
                        "L1_PROPOSER_PRIVATE_KEY".to_string(),
                        l1_proposer_private_key,
                    );

                    env_manager.save().expect("Failed to save .env file");
                } else {
                    println!("No changes made to proposer configuration.");
                    return;
                }
            }
            // Proposer is enabled
            else {
                let selections = &["Disable", "Update", "Cancel"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "The node is already configured as a proposer. What would you like to do?",
                    )
                    .default(0)
                    .items(&selections[..])
                    .interact()
                    .unwrap();

                match selection {
                    0 => {
                        env_manager.set("ENABLE_PROPOSER".to_string(), "false".to_string());
                        env_manager.save().expect("Failed to save .env file");
                        stn_log("Proposer flag set to disabled.");
                    }
                    // TODO: remove code duplication and into a function
                    1 => {
                        // Check if they have a node installed, running, and fully synced
                        let local_http = format!(
                            "http://localhost:{}",
                            env_manager
                                .get("PORT_L2_EXECUTION_ENGINE_HTTP")
                                .expect("PORT_L2_EXECUTION_ENGINE_HTTP not set")
                        );
                        let canonical_http = constants::KATLA_RPC_URL.to_string();
                        let node_synced = network::is_synced(&local_http, &canonical_http).await;
                        if !node_synced {
                            println!("Node is not installed, running, or fully synced.");
                            return;
                        }
                        // Check if they have a local prover running
                        let local_prover_running = env_manager
                            .get("PROVER_ENDPOINTS")
                            .expect("PROVER_ENDPOINTS not set.")
                            .to_string()
                            .contains("taiko_client_prover_relayer");
                        let mut is_local_prover_functional = false;
                        // Local prover set
                        if local_prover_running {
                            is_local_prover_functional =
                                network::is_prover_api_functional(&local_http).await;
                            if !is_local_prover_functional {
                                println!("Local prover is running but the API is not functional.");
                            }
                        }
                        // If local prover running but not functional
                        if local_prover_running && !is_local_prover_functional {
                            // If they don't have a local prover running, ask if they would like to setup a marketplace prover
                            print!("Would you like to setup a marketplace prover? (y/n): ");
                            io::stdout().flush().expect("Failed to flush stdout");
                            let mut input = String::new();
                            io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read input");
                            if input.trim() == "y" {
                                // Healthcheck the marketplace prover
                                if !network::is_prover_api_functional(constants::DEFAULT_PROVER_URL)
                                    .await
                                {
                                    // If the marketplace prover fails, send them to the docs to find another marketplace prover
                                    println!("Marketplace prover failed. Please consult the documentation to find another marketplace prover and manually update the .env.");
                                    return;
                                } else {
                                    // If the marketplace prover succeeds, set the variable in their .env with env_manager
                                    env_manager.set(
                                        "PROVER_ENDPOINTS".to_string(),
                                        constants::DEFAULT_PROVER_URL.to_string(),
                                    );
                                    stn_log(&format!(
                                        "Set {} as marketplace prover.",
                                        constants::DEFAULT_PROVER_URL
                                    ));
                                }
                            } else {
                                println!("No changes made to proposer configuration.");
                                return;
                            }
                        }
                        // Marketplace prover running but not functional
                        if !local_prover_running
                            && !network::is_prover_api_functional(constants::DEFAULT_PROVER_URL)
                                .await
                        {
                            println!("Marketplace prover failed. Please consult the documentation to find another marketplace prover. No changes made to proposer configuration.");
                            return;
                        }
                        // Now prompt and set their proposer private key
                        let l1_proposer_private_key: String = dialoguer::Password::new()
                        .with_prompt(
                            "Please enter your L1_PROPOSER_PRIVATE_KEY (use a test account only!)",
                        )
                        .interact()
                        .expect("Failed to read L1_PROPOSER_PRIVATE_KEY");

                        // Now that their prover is set, and functional, and we have verified they are fully synced, proceed to enable_proposer flag to true, and set private key
                        env_manager.set("ENABLE_PROPOSER".to_string(), "true".to_string());
                        env_manager.set(
                            "L1_PROPOSER_PRIVATE_KEY".to_string(),
                            l1_proposer_private_key,
                        );

                        env_manager.save().expect("Failed to save .env file");
                    }
                    2 => {
                        println!("No changes made to proposer configuration.");
                        return;
                    }
                    _ => {} // This case should never happen as we have only 3 options
                }
            }

            // Offer to restart the node to apply changes
            print!("Would you like to restart the node to apply changes? (y/n): ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut restart_input = String::new();
            io::stdin()
                .read_line(&mut restart_input)
                .expect("Failed to read input");
            if restart_input.trim() == "y" {
                restart(taiko_node_dir).await;
            } else {
                println!("Changes will take effect after the next restart.");
            }
        }
        ConfigSubcommands::Zkp => {
            stn_log("ZKP setup coming soon.");
        }
        ConfigSubcommands::Sgx => {
            stn_log("Sgx setup coming soon.");
        }
    }
}
