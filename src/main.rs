use std::io;
use actix_cors::Cors;
use actix_web::{dev::Service as _, middleware, App, HttpServer};
use juniper::futures::FutureExt;
use product_graph::services::{echo, graph};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server on port 8080 ...");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
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
            .configure(graph::setup)
            .configure(echo::setup)
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
