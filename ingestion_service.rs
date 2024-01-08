// ingestion_service.rs

use crate::analytics::Analytics;
use crate::config::IngestionConfig;
use crate::device::{Device, DeviceManager};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;

/// Service responsible for ingesting data from IoT devices.
pub struct IngestionService {
    device_manager: Arc<DeviceManager>,
    analytics: Arc<Analytics>,
    config: IngestionConfig,
}

impl IngestionService {
    /// Creates a new IngestionService with references to the DeviceManager and Analytics services.
    pub fn new(device_manager: Arc<DeviceManager>, analytics: Arc<Analytics>, config: IngestionConfig) -> Self {
        IngestionService {
            device_manager,
            analytics,
            config,
        }
    }

    /// Starts the ingestion service to listen for incoming data from IoT devices.
    pub fn start(&self) {
        let socket = UdpSocket::bind(&self.config.endpoint).expect("Failed to bind to UDP endpoint");
        println!("Ingestion service listening on {}", self.config.endpoint);

        // Clone Arcs to move into the thread
        let device_manager = Arc::clone(&self.device_manager);
        let analytics = Arc::clone(&self.analytics);

        thread::spawn(move || {
            let mut buf = [0; 1024];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((number_of_bytes, src_addr)) => {
                        let received_data = &mut buf[..number_of_bytes];
                        let data_str = String::from_utf8_lossy(received_data);

                        // Parse the data into a HashMap
                        let device_data: HashMap<String, f64> = match serde_json::from_str(&data_str) {
                            Ok(data) => data,
                            Err(e) => {
                                eprintln!("Failed to parse data from {}: {}", src_addr, e);
                                continue;
                            }
                        };

                        // Extract device ID from the data or source address
                        let device_id = src_addr.to_string(); // Placeholder for actual device ID extraction logic

                        // Update device data and analytics
                        Self::update_device(&device_manager, &device_id, device_data.clone());
                        analytics.process_device_data(&device_id, &device_data);
                    }
                    Err(e) => {
                        eprintln!("Couldn't receive a datagram: {}", e);
                    }
                }
            }
        });
    }

    /// Updates the device's data in the DeviceManager.
    fn update_device(device_manager: &Arc<DeviceManager>, device_id: &str, data: HashMap<String, f64>) {
        let mut devices = device_manager.devices.lock().unwrap();
        let device = devices.entry(device_id.to_string()).or_insert_with(|| Device::new(device_id.to_string(), "Unnamed Device".to_string()));
        device.update_data(data);
    }
}

// Note: This is a simplified example and assumes that the data received from the IoT devices is in JSON format.
// In a real-world scenario, you would need to implement proper error handling, data validation, and potentially
// support for different data formats or protocols.
