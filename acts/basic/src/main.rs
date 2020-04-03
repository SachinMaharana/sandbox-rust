#[macro_use]
extern crate log;

use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.3"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(no_param)
            .service(
                web::resource("/resource2/index.html")
                    .wrap(middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"))
                    .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
                    .route(web::get().to(index_async)),
            )
            .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
            .default_service(web::route().to(|| async { "404 Not Found\r\n" }))
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}

#[get("/")]
async fn no_param() -> &'static str {
    "Hello, actix!\r\n"
}

async fn index_async(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    let y = req.match_info();
    println!("{:?}", y);
    "Hello World!\r\n"
}

#[get("/resource1/{name}/index.html")]
async fn index(req: HttpRequest, name: web::Path<String>) -> String {
    info!("REQ: {:?}", req);
    let y = req.match_info();
    println!("{:?}", y);
    format!("Hello: {}!\r\n", name)
}
