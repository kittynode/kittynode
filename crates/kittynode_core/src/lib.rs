pub mod constants;
mod utils;

use utils::get_kittynode_directory;

pub fn install() {
    println!("Select a package to install:");
    println!("1. Taiko");
    println!("2. LibreNode");
    println!("3. KittyNode");

    // Ensure kittynode directory exists
    let kittynode_dir = get_kittynode_directory().unwrap();
    if !kittynode_dir.exists() {
        println!("Creating kittynode directory: {}", kittynode_dir.display());
        std::fs::create_dir_all(&kittynode_dir).unwrap();
    }
}
