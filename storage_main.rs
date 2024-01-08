 tags as requested:

<FINAL_CODE>
```rust
// storage_main.rs

use storage_service::StorageService;
use config::Config;

fn main() {
    // Load the configuration for the storage service.
    let config = Config::new().expect("Failed to load configuration.");

    // Initialize the storage service with the given configuration.
    let mut storage_service = StorageService::new(config);

    // Start the storage service to listen for incoming data.
    storage_service.run().expect("Failed to run the storage service.");
}
```
