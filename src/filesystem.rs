use std::fs;
use std::io;
use std::path::PathBuf;

/// Expand "~/..." into the user’s home directory
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(path.trim_start_matches("~/"));
        }
    }
    PathBuf::from(path)
}

/// Remove a directory if it exists
pub fn remove_dir_if_exists(path: &PathBuf) -> io::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path)?;
        println!("🗑️  Deleted {}", path.display());
    } else {
        println!("⚠️ Directory not found: {}", path.display());
    }
    Ok(())
}
