use actix_web::{route, web, HttpRequest, HttpResponse, Responder};
use serde_json::{json, Value};

/// Echo endpoint
#[route("/echo", method = "GET", method = "POST")]
async fn service(request: HttpRequest, data: web::Json<Value>) -> impl Responder {
    log::info!("Request: {:?}", request);
    log::info!("Data: {:?}", data);

    HttpResponse::Ok().json(json!({
      "success": true,
      "data": {},
    }))
}

pub fn setup(config: &mut web::ServiceConfig) {
    config.service(service);
}
