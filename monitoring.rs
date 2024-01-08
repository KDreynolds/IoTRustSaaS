// monitoring.rs

use crate::device::{Device, DeviceManager};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Monitoring service that keeps track of the status and health of IoT devices.
pub struct Monitoring {
    device_manager: Arc<DeviceManager>,
    device_health: Arc<Mutex<HashMap<String, DeviceHealth>>>,
}

/// Represents the health status of a single IoT device.
#[derive(Debug, Clone)]
pub struct DeviceHealth {
    pub last_update: Instant,
    pub is_online: bool,
}

impl Monitoring {
    /// Creates a new Monitoring service.
    pub fn new(device_manager: Arc<DeviceManager>) -> Self {
        Monitoring {
            device_manager,
            device_health: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Monitors the health of all devices by checking the time since their last update.
    pub fn monitor_devices(&self) {
        let devices = self.device_manager.devices.lock().unwrap();
        let mut device_health = self.device_health.lock().unwrap();

        for (id, device) in devices.iter() {
            let health = device_health.entry(id.clone()).or_insert_with(|| DeviceHealth {
                last_update: Instant::now(),
                is_online: true,
            });

            // Check if the device has sent an update within a predefined interval
            if health.last_update.elapsed() > Duration::from_secs(30) {
                // If not, mark the device as offline
                health.is_online = false;
            } else {
                // Otherwise, ensure the device is marked as online
                health.is_online = true;
            }
        }
    }

    /// Updates the health status of a specific device when new data is received.
    pub fn update_device_health(&self, device_id: &str) {
        let mut device_health = self.device_health.lock().unwrap();
        if let Some(health) = device_health.get_mut(device_id) {
            health.last_update = Instant::now();
            health.is_online = true;
        } else {
            // If the device is not yet in the health map, add it with the current timestamp
            device_health.insert(device_id.to_string(), DeviceHealth {
                last_update: Instant::now(),
                is_online: true,
            });
        }
    }

    /// Retrieves the health status of a specific device.
    pub fn get_device_health(&self, device_id: &str) -> Option<DeviceHealth> {
        let device_health = self.device_health.lock().unwrap();
        device_health.get(device_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_device_health_update() {
        let device_manager = Arc::new(DeviceManager::new());
        let monitoring = Monitoring::new(device_manager.clone());

        let device = Device::new("device1".to_string(), "Test Device".to_string());
        device_manager.add_device(device);

        // Simulate device update
        monitoring.update_device_health("device1");

        // Allow some time to pass
        thread::sleep(Duration::from_secs(1));

        // Check if the device is still considered online
        let health = monitoring.get_device_health("device1").unwrap();
        assert_eq!(health.is_online, true);
    }

    #[test]
    fn test_device_health_monitoring() {
        let device_manager = Arc::new(DeviceManager::new());
        let monitoring = Monitoring::new(device_manager.clone());

        let device = Device::new("device1".to_string(), "Test Device".to_string());
        device_manager.add_device(device);

        // Simulate device update
        monitoring.update_device_health("device1");

        // Allow some time to pass
        thread::sleep(Duration::from_secs(35));

        // Run the monitoring check
        monitoring.monitor_devices();

        // Check if the device is considered offline after the interval
        let health = monitoring.get_device_health("device1").unwrap();
        assert_eq!(health.is_online, false);
    }
}
