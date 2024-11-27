use crate::infra::config::ConfigStore;
use eyre::Result;

/// Adds a capability to the config if it doesn't already exist.
pub fn add_capability(capability: &str) -> Result<()> {
    let mut config = ConfigStore::load()?;
    config.capabilities = add_to_capabilities(config.capabilities, capability);
    ConfigStore::save(&config)?;
    Ok(())
}

/// Helper function to add a capability to the list, ensuring no duplicates.
fn add_to_capabilities(mut capabilities: Vec<String>, capability: &str) -> Vec<String> {
    if !capabilities.contains(&capability.to_string()) {
        capabilities.push(capability.to_string());
    }
    capabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doesnt_add_duplicate() {
        let capabilities = vec!["cap1".to_string(), "cap2".to_string()];
        let updated_capabilities = add_to_capabilities(capabilities.clone(), "cap1");
        assert_eq!(updated_capabilities, capabilities);
    }

    #[test]
    fn adds_new_capability() {
        let capabilities = vec!["cap1".to_string(), "cap2".to_string()];
        let updated_capabilities = add_to_capabilities(capabilities, "cap3");
        assert_eq!(
            updated_capabilities,
            vec!["cap1".to_string(), "cap2".to_string(), "cap3".to_string()]
        );
    }
}
