// lib.rs

// Make the modules public so they can be accessed from outside the library
pub mod config;
pub mod device;
pub mod analytics;
pub mod monitoring;
pub mod ingestion_service;
pub mod storage_service;
pub mod processing_service;
pub mod api_service;

// Re-export the main components of the library for easier access
pub use config::Config;
pub use device::{Device, DeviceManager};
pub use analytics::Analytics;
pub use monitoring::Monitoring;
pub use ingestion_service::IngestionService;
pub use storage_service::StorageService;
pub use processing_service::ProcessingService;
pub use api_service::APIService;

// You might also want to define some shared types or utilities that are used across the modules
// For example, a Result type that is used throughout the library could be defined here
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// If there are any shared constants or configurations that are used across multiple modules,
// they could also be defined or re-exported here.

// Additionally, if the library provides any initialization or setup functions, they could be
// defined here. For example, a function to create and initialize all services might look like this:

/// Initializes all services and returns a tuple of their instances.
pub fn initialize_services(config: Config) -> Result<(IngestionService, StorageService, ProcessingService, APIService)> {
    let device_manager = DeviceManager::new();
    let analytics = Analytics::new();
    let monitoring = Monitoring::new();

    let ingestion_service = IngestionService::new(device_manager.clone(), analytics.clone());
    let storage_service = StorageService::new(config.storage_config, device_manager.clone());
    let processing_service = ProcessingService::new(device_manager, analytics, monitoring);
    let api_service = APIService::new();

    Ok((ingestion_service, storage_service, processing_service, api_service))
}

// Note: The above function assumes that each service has a `new` method that takes the required
// dependencies as parameters. You would need to adjust the parameters and initialization logic
// based on the actual implementation of your services.
