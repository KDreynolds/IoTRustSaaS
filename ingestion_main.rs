// ingestion_main.rs

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use my_iot_platform::{
    Config, DeviceManager, IngestionService, initialize_services, Result
};

fn main() -> Result<()> {
    // Load configuration from a file or environment variables
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".into());
    let config = Config::from_file(config_path.into())?;

    // Initialize services
    let (ingestion_service, _, _, _) = initialize_services(config.clone())?;

    // Start the ingestion service in a separate thread
    let ingestion_service_arc = Arc::new(Mutex::new(ingestion_service));
    let ingestion_thread = {
        let ingestion_service = Arc::clone(&ingestion_service_arc);
        thread::spawn(move || {
            let service = ingestion_service.lock().unwrap();
            service.run();
        })
    };

    // Optionally, handle signals for graceful shutdown
    // ...

    // Wait for the ingestion thread to finish (it might run indefinitely)
    ingestion_thread.join().expect("Ingestion thread has panicked");

    Ok(())
}
