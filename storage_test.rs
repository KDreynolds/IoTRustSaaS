```rust
// storage_test.rs

use crate::config::StorageConfig;
use crate::device::{Device, DeviceManager};
use crate::storage_service::StorageService;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a test storage service with a mock configuration and device manager.
    fn setup_test_storage_service() -> StorageService {
        let mock_config = StorageConfig {
            database_url: "postgres://test_user:test_password@localhost/test_db".to_string(),
            max_connections: 10,
        };
        let device_manager = Arc::new(Mutex::new(DeviceManager::new()));
        StorageService::new(mock_config, device_manager)
    }

    #[test]
    fn test_store_device_data() {
        let storage_service = setup_test_storage_service();
        let device_id = "test_device123".to_string();
        let device_data = HashMap::from([
            ("temperature".to_string(), 25.0),
            ("humidity".to_string(), 50.0),
        ]);

        let result = storage_service.store_device_data(&device_id, device_data);
        assert!(result.is_ok(), "Storing device data should be successful");
    }

    #[test]
    fn test_storage_service_run() {
        let mut storage_service = setup_test_storage_service();

        // We are testing if the service run method works without any errors.
        // In a real-world scenario, we would mock the database connection and
        // assert that data is being sent to the database.
        let result = storage_service.run();
        assert!(result.is_ok(), "Storage service run should be successful");
    }
}
```
