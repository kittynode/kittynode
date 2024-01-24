use std::path::{Path, PathBuf};
use std::{env, io};

use crate::constants;

/// Logs a message with a `stn_log` prefix.
pub fn stn_log(msg: &str) {
    println!("stn_log: {}", msg);
}

/// Helper function that returns the path to the taiko-node directory.
pub fn get_taiko_node_directory() -> Result<PathBuf, io::Error> {
    let home_dir = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Failed to get home directory"))?;
    Ok(Path::new(&home_dir).join(constants::TAIKO_NODE_PATH))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `stn_log` function does not panic when called.
    #[test]
    fn test_stn_log() {
        stn_log("This is a test log message.");
    }
}
