use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    dev::Service as _,
    middleware, route,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use juniper::futures::FutureExt;
use juniper::http::GraphQLRequest;
use product_graph::schema::{create_schema, Schema};
use product_graph::types::{ApplicationContext, ApplicationContextBuilder};
use serde_json::{json, Value};

// use tokio::time::sleep;

/// GraphQL endpoint
#[route("/", method = "GET", method = "POST")]
async fn graphql(
    schema: web::Data<Schema>,
    context: web::Data<ApplicationContext>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let response_data = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(response_data)
}

/// Echo endpoint
#[route("/echo", method = "GET", method = "POST")]
async fn echo(request: HttpRequest, data: web::Json<Value>) -> impl Responder {
    log::info!("Request: {:?}", request);
    log::info!("Data: {:?}", data);

    HttpResponse::Ok().json(json!({
      "success": true,
      "data": {},
    }))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());
    // Build the context
    let context = Arc::new(ApplicationContextBuilder::default().build());

    log::info!("starting HTTP server on port 8080 ...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::from(context.clone()))
            .wrap_fn(|req, srv| {
                if let Some(ip) = req.headers().get("x-forwarded-for") {
                    log::info!("IP: {}", ip.to_str().unwrap());
                }

                srv.call(req).map(|res| {
                    log::info!("Hi from response {}", res.as_ref().unwrap().status());
                    res
                })
            })
            .wrap(Cors::permissive()) //If you want to use Apollo Studio you need this
            .wrap(middleware::Logger::default())
            .service(graphql)
            .service(echo)
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// tokio::spawn(async move {
//     let last_activity = Instant::now();
//     loop {
//         let idle_time = last_activity.clone().elapsed();
//         println!("Idle for {idle_time:?}");
//         if idle_time > Duration::from_secs(60) {
//             println!("Stopping machine. Goodbye!");
//             std::process::exit(0)
//         }
//         sleep(Duration::from_secs(20)).await;
//     }
// });
