pub mod constants;
mod utils;

use utils::get_stn_directory;

pub fn install() {
    println!("Select a package to install:");
    println!("1. Taiko");
    println!("2. LibreNode");
    println!("3. KittyNode");

    // Ensure stn directory exists
    let stn_dir = get_stn_directory().unwrap();
    if !stn_dir.exists() {
        println!("Creating stn directory: {}", stn_dir.display());
        std::fs::create_dir_all(&stn_dir).unwrap();
    }
}
