// config.rs

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;

/// Represents the configuration for the entire platform.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ingestion_config: IngestionConfig,
    pub storage_config: StorageConfig,
    pub processing_config: ProcessingConfig,
    pub api_config: APIConfig,
}

/// Represents the configuration for the ingestion service.
#[derive(Debug, Serialize, Deserialize)]
pub struct IngestionConfig {
    pub endpoint: SocketAddr,
    // Add other relevant configuration options for the ingestion service here
}

/// Represents the configuration for the storage service.
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database_url: String,
    pub max_connections: u32,
    // Add other relevant configuration options for the storage service here
}

/// Represents the configuration for the processing service.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub processing_interval: u64,
    // Add other relevant configuration options for the processing service here
}

/// Represents the configuration for the API service.
#[derive(Debug, Serialize, Deserialize)]
pub struct APIConfig {
    pub api_endpoint: SocketAddr,
    // Add other relevant configuration options for the API service here
}

impl Config {
    /// Loads the configuration from a given file path.
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&config_str)?;
        Ok(config)
    }

    /// Loads the configuration from environment variables.
    pub fn from_env() -> Result<Self> {
        // Use environment variables to create the Config struct
        // This is a placeholder example, you should implement the actual logic
        // to read from environment variables and construct the Config struct.
        Ok(Self {
            ingestion_config: IngestionConfig {
                endpoint: "127.0.0.1:8080".parse().unwrap(),
            },
            storage_config: StorageConfig {
                database_url: "postgres://user:password@localhost/dbname".to_string(),
                max_connections: 10,
            },
            processing_config: ProcessingConfig {
                processing_interval: 1000,
            },
            api_config: APIConfig {
                api_endpoint: "127.0.0.1:3000".parse().unwrap(),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_file() {
        let config_path = PathBuf::from("config.toml");
        let config = Config::from_file(config_path);
        assert!(config.is_ok());
    }

    #[test]
    fn test_config_from_env() {
        let config = Config::from_env();
        assert!(config.is_ok());
    }
}
