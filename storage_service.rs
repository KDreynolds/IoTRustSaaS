```rust
// storage_service.rs

use crate::config::StorageConfig;
use crate::device::{Device, DeviceManager};
use std::sync::{Arc, Mutex};
use std::error::Error;

/// Represents the storage service responsible for persisting device data.
pub struct StorageService {
    config: StorageConfig,
    device_manager: Arc<Mutex<DeviceManager>>,
}

impl StorageService {
    /// Creates a new storage service with the given configuration.
    pub fn new(config: StorageConfig, device_manager: Arc<Mutex<DeviceManager>>) -> Self {
        StorageService {
            config,
            device_manager,
        }
    }

    /// Runs the storage service, listening for incoming data and storing it.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Here you would connect to the database using the provided database URL
        // and max_connections from the config. Since we're not implementing a real
        // database connection in this example, we'll just print the configuration.

        println!("Storage Service is running with the following configuration:");
        println!("Database URL: {}", self.config.database_url);
        println!("Max Connections: {}", self.config.max_connections);

        // The storage service would typically listen for incoming data from the
        // ingestion service and store it in the database. For this example, we'll
        // simulate storing data for a single device.

        // Simulate receiving data for a device
        let device_id = "device123".to_string();
        let device_data = std::collections::HashMap::from([
            ("temperature".to_string(), 22.5),
            ("humidity".to_string(), 45.0),
        ]);

        // Store the data
        self.store_device_data(&device_id, device_data)?;

        Ok(())
    }

    /// Stores the data for a specific device.
    fn store_device_data(&self, device_id: &str, data: std::collections::HashMap<String, f64>) -> Result<(), Box<dyn Error>> {
        let mut device_manager = self.device_manager.lock().unwrap();
        device_manager.update_device_data(device_id, data);

        // Here you would typically insert the data into the database.
        // For this example, we'll just print the data that would be stored.

        println!("Storing data for device {}: {:?}", device_id, data);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_storage_service_run() {
        let config = Config::from_env().unwrap();
        let device_manager = Arc::new(Mutex::new(DeviceManager::new()));
        let mut storage_service = StorageService::new(config.storage_config, device_manager);

        let result = storage_service.run();
        assert!(result.is_ok());
    }
}
```
