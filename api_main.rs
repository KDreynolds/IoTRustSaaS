use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::Arc;
use crate::api_service::{ApiService, ApiServiceImpl};
use crate::config::Config;
use crate::storage_service::StorageService;
use crate::processing_service::ProcessingService;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Service is up and running")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Arc::new(Config::new());
    let storage_service = Arc::new(StorageService::new(config.clone()));
    let processing_service = Arc::new(ProcessingService::new(config.clone()));
    let api_service = Arc::new(ApiServiceImpl::new(storage_service, processing_service));

    HttpServer::new(move || {
        App::new()
            .data(api_service.clone())
            .route("/health", web::get().to(health_check))
            // Define other routes here
            .service(
                web::scope("/api")
                    // Add API endpoints here
                    // For example, to get data from a specific device:
                    .route("/devices/{device_id}", web::get().to(ApiService::get_device_data))
                    // To get analytics data:
                    .route("/analytics", web::get().to(ApiService::get_analytics_data))
                    // To monitor devices in real-time:
                    .route("/monitor", web::get().to(ApiService::monitor_devices))
            )
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

// You would also define the necessary handlers for the routes here, or in a separate module.
// For example:
// async fn get_device_data(info: web::Path<(u32,)>, api: web::Data<Arc<ApiServiceImpl>>) -> impl Responder {
//     let device_id = info.into_inner();
//     let data = api.get_device_data(device_id).await;
//     match data {
//         Ok(data) => HttpResponse::Ok().json(data),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
