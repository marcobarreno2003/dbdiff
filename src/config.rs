use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_DIR: &str = ".dbdiff";
const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connection_string: String,
}

impl Config {
    pub fn new(connection_string: String) -> Self {
        Self { connection_string }
    }

    /// Get the .dbdiff directory path
    pub fn dir_path() -> PathBuf {
        PathBuf::from(CONFIG_DIR)
    }

    /// Get the config file path
    pub fn file_path() -> PathBuf {
        Self::dir_path().join(CONFIG_FILE)
    }

    /// Get the SQLite database path
    pub fn db_path() -> PathBuf {
        Self::dir_path().join("snapshots.db")
    }

    /// Check if dbdiff is initialized in current directory
    pub fn is_initialized() -> bool {
        Self::file_path().exists()
    }

    /// Initialize dbdiff in current directory
    pub fn init(connection_string: String) -> Result<Self> {
        let config = Self::new(connection_string);

        // Create .dbdiff directory
        std::fs::create_dir_all(Self::dir_path())
            .context("Failed to create .dbdiff directory")?;

        // Save config
        config.save()?;

        Ok(config)
    }

    /// Load config from file
    pub fn load() -> Result<Self> {
        let path = Self::file_path();

        if !path.exists() {
            anyhow::bail!(
                "dbdiff not initialized. Run 'dbdiff init -c <connection_string>' first."
            );
        }

        let content = std::fs::read_to_string(&path)
            .context("Failed to read config file")?;

        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let path = Self::file_path();
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&path, content)
            .context("Failed to write config file")?;

        Ok(())
    }
}
