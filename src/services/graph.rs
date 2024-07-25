pub mod schema;

use std::sync::Arc;

use crate::types::ApplicationContext;
use juniper::http::GraphQLRequest;
use schema::Schema;

use actix_web::{
    route,
    web::{self, Data},
    HttpResponse, Responder,
};

use self::schema::create_schema;

// GraphQL endpoint
#[route("/", method = "GET", method = "POST")]
async fn service(
    schema: Data<Schema>,
    context: Data<ApplicationContext>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let response_data = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(response_data)
}

pub fn setup(config: &mut web::ServiceConfig, context: ApplicationContext) {
    // Create Juniper schema

    let schema = Arc::new(create_schema());
    config
        .app_data(Data::from(schema.clone()))
        .app_data(Data::from(Arc::new(context)))
        .service(service);
}
