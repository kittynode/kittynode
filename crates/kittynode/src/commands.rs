use eyre::Result;
use kittynode_core::package::get_packages;

pub async fn get_packages_command() -> Result<()> {
    let packages = get_packages()?;
    for (name, package) in packages {
        println!("Package: {}\n{}", name, package);
    }
    Ok(())
}
