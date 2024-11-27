use eyre::Result;
use kittynode_core::application::delete_package;

pub async fn delete_package_cmd(name: String, include_images: bool) -> Result<()> {
    delete_package(&name, include_images).await
}
