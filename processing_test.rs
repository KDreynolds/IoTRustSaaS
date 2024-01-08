```rust
// processing_test.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ProcessingConfig};
    use crate::device::{Device, DeviceManager};
    use crate::analytics::Analytics;
    use crate::monitoring::{Monitoring, DeviceHealth};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    // Helper function to create a test device with some dummy data
    fn create_test_device(id: &str, name: &str) -> Device {
        let mut data = HashMap::new();
        data.insert("temperature".to_string(), 22.5);
        data.insert("humidity".to_string(), 45.0);
        Device::new(id.to_string(), name.to_string())
    }

    // Helper function to create a ProcessingService with test data
    fn setup_processing_service() -> ProcessingService {
        let device_manager = Arc::new(DeviceManager::new());
        let analytics = Arc::new(Analytics::new(device_manager.clone()));
        let monitoring = Arc::new(Monitoring::new(device_manager.clone()));

        let config = Config {
            processing_config: ProcessingConfig {
                processing_interval: 1000, // 1 second for testing
            },
            // Other config structs would be initialized here
            ..Default::default()
        };

        ProcessingService::new(device_manager, analytics, monitoring, config.processing_config)
    }

    #[test]
    fn test_processing_service_initialization() {
        let processing_service = setup_processing_service();
        assert!(processing_service.is_some(), "ProcessingService should be successfully initialized");
    }

    #[test]
    fn test_device_data_processing() {
        let processing_service = setup_processing_service().unwrap();
        let test_device = create_test_device("test_device_id", "Test Device");

        // Simulate device data ingestion
        processing_service.device_manager.add_device(test_device.clone());
        processing_service.device_manager.update_device_data(&test_device.id, test_device.data.clone());

        // Process the data
        processing_service.process_data();

        // Check if the analytics and monitoring services have been updated
        let analytics_data = processing_service.analytics.update_counts.lock().unwrap();
        let monitoring_data = processing_service.monitoring.device_health.lock().unwrap();

        assert_eq!(analytics_data.get(&test_device.id), Some(&1), "Analytics should have processed the device data once");
        assert!(monitoring_data.get(&test_device.id).is_some(), "Monitoring should have a health record for the device");

        let device_health = monitoring_data.get(&test_device.id).unwrap();
        assert!(device_health.is_online, "Device should be marked as online after processing");
        assert!(device_health.last_update.elapsed() < Duration::from_secs(2), "Device last update should be recent");
    }
}
```
