use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use crate::analytics;
use crate::monitoring;
use crate::device::Device;

// Define a struct for the API state that might include references to analytics and monitoring services
pub struct ApiService {
    analytics: analytics::AnalyticsService,
    monitoring: monitoring::MonitoringService,
}

// Define request and response data structures
#[derive(Serialize, Deserialize)]
struct DeviceDataRequest {
    device_id: String,
}

#[derive(Serialize, Deserialize)]
struct DeviceDataResponse {
    device_id: String,
    data: Vec<Device>,
}

// Define API endpoints
async fn get_device_data(info: web::Json<DeviceDataRequest>, api: web::Data<ApiService>) -> impl Responder {
    let device_data = api.analytics.get_device_data(&info.device_id).await;
    HttpResponse::Ok().json(DeviceDataResponse {
        device_id: info.device_id.clone(),
        data: device_data,
    })
}

async fn get_monitoring_data(api: web::Data<ApiService>) -> impl Responder {
    let monitoring_data = api.monitoring.get_monitoring_data().await;
    HttpResponse::Ok().json(monitoring_data)
}

// Configure the API service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/device_data")
            .route(web::post().to(get_device_data))
    );
    cfg.service(
        web::resource("/monitoring_data")
            .route(web::get().to(get_monitoring_data))
    );
}

// Start the API server
pub async fn start_api_server(api_service: ApiService) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(api_service.clone())
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
