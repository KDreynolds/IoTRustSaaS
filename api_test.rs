```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, http};
    use crate::config::APIConfig;
    use crate::device::DeviceManager;
    use crate::analytics::Analytics;
    use crate::monitoring::Monitoring;
    use std::sync::Arc;

    // Setup a test server with the ApiService
    fn setup_api_service() -> ApiService {
        let device_manager = Arc::new(DeviceManager::new());
        let analytics = Analytics::new(device_manager.clone());
        let monitoring = Monitoring::new(device_manager);

        ApiService {
            analytics,
            monitoring,
        }
    }

    #[actix_rt::test]
    async fn test_get_device_data() {
        let api_service = setup_api_service();
        let mut app = test::init_service(
            App::new()
                .data(api_service)
                .configure(api_service::config)
        ).await;

        let req = test::TestRequest::post()
            .uri("/device_data")
            .set_json(&DeviceDataRequest { device_id: "device123".to_string() })
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let result: DeviceDataResponse = test::read_body_json(resp).await;
        assert_eq!(result.device_id, "device123");
    }

    #[actix_rt::test]
    async fn test_get_monitoring_data() {
        let api_service = setup_api_service();
        let mut app = test::init_service(
            App::new()
                .data(api_service)
                .configure(api_service::config)
        ).await;

        let req = test::TestRequest::get()
            .uri("/monitoring_data")
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Assuming the monitoring data is a HashMap<String, DeviceHealth>
        let result: HashMap<String, DeviceHealth> = test::read_body_json(resp).await;
        // Here you would add assertions to check if the monitoring data is as expected
        // For example:
        // assert!(result.contains_key("device123"));
    }
}
```
