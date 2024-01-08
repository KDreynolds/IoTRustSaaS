#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_device_creation() {
        let device = Device::new("device1".to_string(), "Temperature Sensor".to_string());
        assert_eq!(device.id, "device1");
        assert_eq!(device.name, "Temperature Sensor");
        assert!(device.data.is_empty());
    }

    #[test]
    fn test_device_data_update() {
        let mut device = Device::new("device2".to_string(), "Humidity Sensor".to_string());
        let mut data = HashMap::new();
        data.insert("humidity".to_string(), 42.5);
        device.update_data(data.clone());
        assert_eq!(device.data, data);
    }

    #[test]
    fn test_device_manager_add_and_get_device() {
        let device_manager = DeviceManager::new();
        let device = Device::new("device3".to_string(), "Pressure Sensor".to_string());
        device_manager.add_device(device.clone()).unwrap();
        let retrieved_device = device_manager.get_device(&"device3".to_string()).unwrap();
        assert_eq!(retrieved_device.id, device.id);
        assert_eq!(retrieved_device.name, device.name);
    }

    #[test]
    fn test_analytics_process_device_data() {
        let device_manager = Arc::new(DeviceManager::new());
        let analytics = Analytics::new(device_manager.clone());
        let device_id = "device4".to_string();
        let data = HashMap::new();
        analytics.process_device_data(&device_id, &data);
        let update_counts = analytics.update_counts.lock().unwrap();
        assert_eq!(*update_counts.get(&device_id).unwrap(), 1);
    }

    #[test]
    fn test_monitoring_update_device_health() {
        let device_manager = Arc::new(DeviceManager::new());
        let monitoring = Monitoring::new(device_manager.clone());
        let device_id = "device5".to_string();
        let device_health = DeviceHealth {
            last_update: std::time::Instant::now(),
            is_online: true,
        };
        monitoring.update_device_health(&device_id, device_health.clone()).unwrap();
        let retrieved_health = monitoring.get_device_health(&device_id).unwrap();
        assert_eq!(retrieved_health.is_online, device_health.is_online);
    }

    #[test]
    fn test_initialize_services() {
        let config = Config {
            ingestion_config: IngestionConfig {
                endpoint: "127.0.0.1:8080".parse().unwrap(),
            },
            storage_config: StorageConfig {
                database_url: "sqlite::memory:".to_string(),
                max_connections: 5,
            },
            processing_config: ProcessingConfig {
                processing_interval: 1000,
            },
            api_config: APIConfig {
                api_endpoint: "127.0.0.1:8081".parse().unwrap(),
            },
        };

        let (ingestion_service, storage_service, processing_service, api_service) =
            initialize_services(config).unwrap();

        // Here you would write tests to ensure that the services are initialized correctly
        // For example, you could check if they are listening on the correct endpoints,
        // or if they have the correct dependencies set up.
        // Since this is a simple test, we'll just check that they are not null.

        assert!(Arc::ptr_eq(
            &ingestion_service.device_manager,
            &device_manager
        ));
        assert_eq!(storage_service.config.database_url, "sqlite::memory:");
        assert_eq!(processing_service.config.processing_interval, 1000);
        // API service test would be similar to the above, checking its configuration
    }
}
