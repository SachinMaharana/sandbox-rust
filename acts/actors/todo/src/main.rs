#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use tera::Tera;

use std::{env, io};

mod api;
mod db;
mod schema;

static SESSION_KEY: &[u8] = &[0; 32];

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_db(&database_url).expect("Failed to create pool");

    let app = move || {
        debug!("Launching App");
        let templates: Tera = Tera::new("templates/**/*").unwrap();
        let session_store = CookieSession::signed(SESSION_KEY).secure(false);

        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::internal_server_error,
            )
            .handler(http::StatusCode::NOT_FOUND, api::not_found)
            .handler(http::StatusCode::BAD_REQUEST, api::bad_request);
        App::new()
            .data(templates)
            .data(pool.clone())
            .wrap(session_store)
            .wrap(error_handlers)
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(web::resource("/todo").route(web::post().to(api::create)))
            .service(web::resource("/todo/{id}").route(web::get().to(api::update)))
            .service(fs::Files::new("/static", "static/"))
    };
    debug!("Starting Server");

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
