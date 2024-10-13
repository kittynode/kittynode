use eyre::{Context, Result};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use tracing::info;

pub(crate) fn kittynode_path() -> Result<PathBuf> {
    let home_dir = env::var("HOME").wrap_err("Failed to read HOME environment variable")?;
    Ok(Path::new(&home_dir).join(".kittynode"))
}

pub(crate) fn init_kittynode_dir() -> Result<()> {
    let path = kittynode_path()?;

    if !path.exists() {
        info!("Creating .kittynode directory");
        fs::create_dir_all(&path).wrap_err("Failed to create .kittynode directory")?;
    }

    Ok(())
}

pub(crate) fn generate_jwt_secret() -> Result<String> {
    init_kittynode_dir()?;

    let output = std::process::Command::new("openssl")
        .args(["rand", "-hex", "32"])
        .output()
        .wrap_err("Failed to generate JWT secret with openssl")?;
    let secret = String::from_utf8(output.stdout)?.trim().to_string();

    let path = kittynode_path()?;
    fs::write(path.join("jwt.hex"), &secret).wrap_err("Failed to write JWT secret to file")?;

    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn it_creates_the_kittynode_directory() {
        let temp_dir = tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());

        let result = init_kittynode_dir();
        assert!(result.is_ok());

        let expected_path = temp_dir.path().join(".kittynode");
        assert!(expected_path.exists());
    }

    #[test]
    fn it_generates_a_jwt_secret() {
        let temp_dir = tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());

        let result = generate_jwt_secret();
        assert!(result.is_ok());

        // Verify jwt.hex file exists at the path
        let jwt_file_path = temp_dir.path().join(".kittynode").join("jwt.hex");
        assert!(jwt_file_path.exists());

        // Verify the content of the jwt.hex file
        let secret = fs::read_to_string(jwt_file_path).unwrap();
        assert_eq!(secret.len(), 64); // 32 bytes in hex = 64 characters
        assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));

        // Verify that the returned secret matches the file content
        assert_eq!(result.unwrap(), secret);
    }
}
