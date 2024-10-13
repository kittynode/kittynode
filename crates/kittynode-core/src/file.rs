use eyre::{Context, Result};
use std::{env, fs, path::PathBuf};
use tracing::info;

pub(crate) fn kittynode_path() -> Result<PathBuf> {
    env::var("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".kittynode"))
        .wrap_err("Failed to determine the .kittynode path")
}

pub(crate) fn generate_jwt_secret() -> Result<String> {
    let path = kittynode_path()?;

    if !path.exists() {
        info!("Creating .kittynode directory at {:?}", path);
        fs::create_dir_all(&path).wrap_err("Failed to create .kittynode directory")?;
    }

    info!("Generating JWT secret using OpenSSL");
    let output = std::process::Command::new("openssl")
        .args(["rand", "-hex", "32"])
        .output()
        .wrap_err("Failed to generate JWT secret with openssl")?;

    if !output.status.success() {
        return Err(eyre::eyre!("openssl command failed: {:?}", output));
    }

    let secret = String::from_utf8(output.stdout)?.trim().to_string();
    fs::write(path.join("jwt.hex"), &secret).wrap_err("Failed to write JWT secret to file")?;

    info!(
        "JWT secret successfully generated and written to {:?}",
        path.join("jwt.hex")
    );
    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn it_generates_a_jwt_secret() {
        let temp_dir = tempdir().unwrap();
        env::set_var("HOME", temp_dir.path().to_str().unwrap());

        let result = generate_jwt_secret();
        assert!(result.is_ok(), "Expected OK, got {:?}", result);

        let jwt_file_path = temp_dir.path().join(".kittynode").join("jwt.hex");
        assert!(jwt_file_path.exists(), "JWT secret file not found");

        let secret = fs::read_to_string(jwt_file_path).unwrap();
        assert_eq!(secret.len(), 64, "Expected 64 hex characters");
        assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));

        assert_eq!(result.unwrap(), secret, "Secrets do not match");
    }
}
