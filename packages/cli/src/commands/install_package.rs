use eyre::Result;
use kittynode_core::application::install_package;

pub async fn install_package_cmd(name: String) -> Result<()> {
    install_package(&name).await
}
