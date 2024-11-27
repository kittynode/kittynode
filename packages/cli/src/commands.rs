use eyre::Result;
use kittynode_core::application::{delete_package, get_packages, install_package};

pub async fn get_packages_cmd() -> Result<()> {
    let packages = get_packages()?;
    for (name, package) in packages {
        println!("Package: {}\n{}", name, package);
    }
    Ok(())
}

pub async fn install_package_cmd(name: String) -> Result<()> {
    install_package(&name).await
}

pub async fn delete_package_cmd(name: String, include_images: bool) -> Result<()> {
    delete_package(&name, include_images).await
}
