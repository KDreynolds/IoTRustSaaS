```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::IngestionConfig;
    use crate::device::{Device, DeviceManager};
    use crate::analytics::Analytics;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    fn setup_ingestion_service() -> IngestionService {
        let device_manager = Arc::new(DeviceManager::new());
        let analytics = Arc::new(Analytics::new());
        let config = IngestionConfig {
            endpoint: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 12345),
            // ... other config options
        };

        IngestionService::new(device_manager, analytics, config)
    }

    #[test]
    fn test_ingestion_service_creation() {
        let ingestion_service = setup_ingestion_service();
        assert_eq!(ingestion_service.config.endpoint.port(), 12345);
    }

    #[test]
    fn test_ingestion_service_device_registration() {
        let ingestion_service = setup_ingestion_service();
        let device_manager = ingestion_service.device_manager.clone();
        let device = Device::new("device1".to_string(), "Temperature Sensor".to_string());

        {
            let mut devices = device_manager.devices.lock().unwrap();
            devices.insert(device.id.clone(), device);
        }

        let devices = device_manager.devices.lock().unwrap();
        assert!(devices.contains_key("device1"));
    }

    #[test]
    fn test_ingestion_service_data_ingestion() {
        let ingestion_service = setup_ingestion_service();
        let device_manager = ingestion_service.device_manager.clone();
        let mut device = Device::new("device1".to_string(), "Temperature Sensor".to_string());

        let new_data = HashMap::from([
            ("temperature".to_string(), 22.5),
            ("humidity".to_string(), 45.0),
        ]);

        device.update_data(new_data.clone());

        {
            let mut devices = device_manager.devices.lock().unwrap();
            devices.insert(device.id.clone(), device);
        }

        let devices = device_manager.devices.lock().unwrap();
        let device = devices.get("device1").unwrap();
        assert_eq!(device.data.get("temperature"), Some(&22.5));
        assert_eq!(device.data.get("humidity"), Some(&45.0));
    }

    // Additional tests for other ingestion service functionality can be added here
}
```
