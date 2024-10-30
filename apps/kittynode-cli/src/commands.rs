use eyre::Result;
use kittynode_core::package::get_packages;

pub async fn get_packages_command() -> Result<()> {
    let packages = get_packages()?;
    for (name, package) in packages {
        println!("Package: {}\n{}", name, package);
    }
    Ok(())
}

pub async fn install_package(name: String) -> Result<()> {
    kittynode_core::package::install_package(&name).await?;
    Ok(())
}

pub async fn delete_package(name: String) -> Result<()> {
    // Does not include images in deletion
    kittynode_core::package::delete_package(&name, false).await?;
    Ok(())
}
