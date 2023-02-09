use std::{
    io,
    sync::Arc,
    time::{Duration, Instant},
};

use actix_cors::Cors;
use actix_web::{
    dev::Service as _,
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::futures::FutureExt;
use juniper::http::GraphQLRequest;
use product_graph::schema::{create_schema, Schema};
use tokio::time::sleep;

/// GraphQL endpoint
#[route("/", method = "GET", method = "POST")]
async fn graphql(schema: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let response_data = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(response_data)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let last_activity: Arc<Instant> = Arc::new(Instant::now());

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 8080 ...");

    tokio::spawn(async move {
        loop {
            let idle_time = last_activity.elapsed();
            println!("Idle for {idle_time:?}");
            if idle_time > Duration::from_secs(60) {
                println!("Stopping machine. Goodbye!");
                std::process::exit(0)
            }
            sleep(Duration::from_secs(20)).await;
        }
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .wrap_fn(|req, srv| {
                if let Some(ip) = req.headers().get("x-forwarded-for") {
                    println!("IP: {}", ip.to_str().unwrap())
                }
                // Arc::new(last_activity).store(Instant::now(), std::sync::atomic::Ordering::Relaxed);
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .service(graphql)
            .wrap(Cors::permissive()) //If you want to use Apollo Studio you need this
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
