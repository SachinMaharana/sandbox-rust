use actix_web::http::ContentEncoding;
use actix_web::{middleware, web, App, HttpServer};

mod config;
mod db;
mod middlewares;
mod models;
mod repositories;

use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::new(ContentEncoding::Br))
            .wrap(middleware::Logger::default())
            .service(web::scope("/user").configure(repositories::user_repository::init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
