// analytics.rs

use crate::device::{Device, DeviceManager};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Analytics service that processes and analyzes data from IoT devices.
pub struct Analytics {
    device_manager: Arc<DeviceManager>,
    // Here we might store historical data, statistical models, or other analytical data structures
    // For simplicity, we'll just keep a count of the number of updates per device
    update_counts: Arc<Mutex<HashMap<String, u64>>>,
}

impl Analytics {
    /// Creates a new Analytics service.
    pub fn new(device_manager: Arc<DeviceManager>) -> Self {
        Analytics {
            device_manager,
            update_counts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Processes incoming data for a device and updates analytics.
    pub fn process_device_data(&self, device_id: &str, data: &HashMap<String, f64>) {
        let mut update_counts = self.update_counts.lock().unwrap();
        let count = update_counts.entry(device_id.to_string()).or_insert(0);
        *count += 1;

        // Here you would implement more complex analytics, such as:
        // - Aggregating data over time
        // - Running statistical analysis
        // - Detecting anomalies or patterns
        // - Generating insights or predictions based on the data

        // For now, we're just updating a simple count of updates per device
    }

    /// Retrieves the analytics data for a specific device.
    pub fn get_device_analytics(&self, device_id: &str) -> Option<u64> {
        let update_counts = self.update_counts.lock().unwrap();
        update_counts.get(device_id).cloned()
    }

    /// Retrieves analytics data for all devices.
    pub fn get_all_analytics(&self) -> HashMap<String, u64> {
        let update_counts = self.update_counts.lock().unwrap();
        update_counts.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_process_device_data() {
        let device_manager = Arc::new(DeviceManager::new());
        let analytics = Analytics::new(device_manager.clone());

        let device_id = "device_1".to_string();
        let mut data = HashMap::new();
        data.insert("temperature".to_string(), 22.5);

        analytics.process_device_data(&device_id, &data);

        let device_update_count = analytics.get_device_analytics(&device_id);
        assert_eq!(device_update_count, Some(1));
    }

    #[test]
    fn test_get_all_analytics() {
        let device_manager = Arc::new(DeviceManager::new());
        let analytics = Analytics::new(device_manager.clone());

        let device_id_1 = "device_1".to_string();
        let device_id_2 = "device_2".to_string();
        let mut data_1 = HashMap::new();
        let mut data_2 = HashMap::new();
        data_1.insert("temperature".to_string(), 22.5);
        data_2.insert("humidity".to_string(), 55.0);

        analytics.process_device_data(&device_id_1, &data_1);
        analytics.process_device_data(&device_id_2, &data_2);

        let all_analytics = analytics.get_all_analytics();
        assert_eq!(all_analytics.len(), 2);
        assert_eq!(all_analytics.get(&device_id_1), Some(&1));
        assert_eq!(all_analytics.get(&device_id_2), Some(&1));
    }
}
