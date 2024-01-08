// device.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Represents a single IoT device with its associated data.
#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub data: HashMap<String, f64>, // Assuming devices send data as key-value pairs
}

impl Device {
    /// Creates a new IoT device with a unique identifier and name.
    pub fn new(id: String, name: String) -> Self {
        Device {
            id,
            name,
            data: HashMap::new(),
        }
    }

    /// Updates the device's data with new key-value pairs.
    pub fn update_data(&mut self, new_data: HashMap<String, f64>) {
        for (key, value) in new_data.into_iter() {
            self.data.insert(key, value);
        }
    }
}

/// Manages a collection of IoT devices.
pub struct DeviceManager {
    devices: Arc<Mutex<HashMap<String, Device>>>,
}

impl DeviceManager {
    /// Creates a new DeviceManager with an empty collection of devices.
    pub fn new() -> Self {
        DeviceManager {
            devices: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Adds a new device to the manager.
    pub fn add_device(&mut self, device: Device) {
        let mut devices = self.devices.lock().unwrap();
        devices.insert(device.id.clone(), device);
    }

    /// Removes a device from the manager by its unique identifier.
    pub fn remove_device(&mut self, device_id: &str) {
        let mut devices = self.devices.lock().unwrap();
        devices.remove(device_id);
    }

    /// Retrieves a device by its unique identifier.
    pub fn get_device(&self, device_id: &str) -> Option<Device> {
        let devices = self.devices.lock().unwrap();
        devices.get(device_id).cloned()
    }

    /// Updates the data for a specific device.
    pub fn update_device_data(&mut self, device_id: &str, new_data: HashMap<String, f64>) {
        let mut devices = self.devices.lock().unwrap();
        if let Some(device) = devices.get_mut(device_id) {
            device.update_data(new_data);
        }
    }

    /// Returns a list of all devices.
    pub fn list_devices(&self) -> Vec<Device> {
        let devices = self.devices.lock().unwrap();
        devices.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_device() {
        let mut manager = DeviceManager::new();
        let device = Device::new("device1".to_string(), "Temperature Sensor".to_string());
        manager.add_device(device.clone());

        let retrieved_device = manager.get_device("device1");
        assert_eq!(retrieved_device, Some(device));
    }

    #[test]
    fn test_remove_device() {
        let mut manager = DeviceManager::new();
        let device = Device::new("device1".to_string(), "Temperature Sensor".to_string());
        manager.add_device(device);

        manager.remove_device("device1");
        let retrieved_device = manager.get_device("device1");
        assert_eq!(retrieved_device, None);
    }

    #[test]
    fn test_update_device_data() {
        let mut manager = DeviceManager::new();
        let mut device = Device::new("device1".to_string(), "Temperature Sensor".to_string());
        device.data.insert("temperature".to_string(), 25.0);
        manager.add_device(device);

        let new_data = HashMap::from([("temperature".to_string(), 26.5)]);
        manager.update_device_data("device1", new_data);

        let retrieved_device = manager.get_device("device1").unwrap();
        assert_eq!(retrieved_device.data["temperature"], 26.5);
    }

    #[test]
    fn test_list_devices() {
        let mut manager = DeviceManager::new();
        let device1 = Device::new("device1".to_string(), "Temperature Sensor 1".to_string());
        let device2 = Device::new("device2".to_string(), "Temperature Sensor 2".to_string());
        manager.add_device(device1);
        manager.add_device(device2);

        let devices = manager.list_devices();
        assert_eq!(devices.len(), 2);
    }
}
