use eyre::Result;
use kittynode_core::application::get_packages::get_packages;

pub async fn get_packages_command() -> Result<()> {
    let packages = get_packages()?;
    for (name, package) in packages {
        println!("Package: {}\n{}", name, package);
    }
    Ok(())
}

pub async fn install_package(name: String) -> Result<()> {
    kittynode_core::application::install_package::install_package(&name).await?;
    Ok(())
}

pub async fn delete_package(name: String, include_images: bool) -> Result<()> {
    kittynode_core::application::delete_package::delete_package(&name, include_images).await?;
    Ok(())
}
