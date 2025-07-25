use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use product_graph::{
    services::{echo, graph},
    types::ApplicationContextBuilder,
};
use std::io;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let context = ApplicationContextBuilder::default().build().await.unwrap();

    log::info!("starting HTTP server on port 8080 ...");
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) //If you want to use Apollo Studio, you need this
            .wrap(middleware::Logger::default())
            .configure(|config| graph::setup(config, context.clone()))
            .configure(echo::setup)
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
