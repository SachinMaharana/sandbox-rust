use actix_web::{middleware, web, App, HttpRequest, HttpServer};


async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}
#[actix_rt::main]
#[cfg(unix)]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/index.html")
                    .route(web::get().to(|| async { "Hello Okay!" })),
            )
            .service(web::resource("/").to(index))
    })
    .bind_uds("/tmp/actix-uds.socket")?
    .run()
    .await
}