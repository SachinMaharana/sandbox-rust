use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};

use r2d2_sqlite::{self, SqliteConnectionManager};
use std::io;

mod db;

use db::{Pool, Queries};
#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("weather.db");
    let pool = Pool::new(manager).unwrap();

    println!("Serving!");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/asyncio_weather").route(web::get().to(asyncio_weather)))
            .service(web::resource("/healthz").route(web::get().to(healthz)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn asyncio_weather(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    let result = vec![
        db::execute(&db, Queries::GetTopTenHottestYears).await?,
        db::execute(&db, Queries::GetTopTenColdestYears).await?,
        db::execute(&db, Queries::GetTopTenHottestMonths).await?,
        db::execute(&db, Queries::GetTopTenColdestMonths).await?,
    ];
    Ok(HttpResponse::Ok().json(result))
}

async fn healthz() -> Result<HttpResponse, AWError> {
    Ok(HttpResponse::Ok().json({}))
}
