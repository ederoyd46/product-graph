pub mod schema;

use std::sync::Arc;

use crate::types::{ApplicationContext, ApplicationContextBuilder};
use juniper::http::GraphQLRequest;
use schema::Schema;

use actix_web::{
    route,
    web::{self, Data},
    App, HttpResponse, Responder,
};

use self::schema::create_schema;

/// GraphQL endpoint
#[route("/", method = "GET", method = "POST")]
async fn service(
    schema: web::Data<Schema>,
    context: web::Data<ApplicationContext>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let response_data = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(response_data)
}

pub fn setup(config: &mut web::ServiceConfig) {
    // Create Juniper schema
    let schema = Arc::new(create_schema());
    // Build the context
    let context = Arc::new(ApplicationContextBuilder::default().build());

    config
        .app_data(Data::from(schema.clone()))
        .app_data(Data::from(context.clone()))
        .service(service);
}
