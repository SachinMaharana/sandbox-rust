use actix_web::{
    client::Client,
    error::ErrorBadRequest,
    web::{self, BytesMut},
    App, Error, HttpResponse, HttpServer,
};

use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
struct SomeData {}

async fn create_something(
    some_data: web::Json<SomeData>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    unimplemented!()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let endpoint = "127.0.0.1:8080";

    println!("Starting Server at: {:?}", endpoint);

    HttpServer::new(|| {
        App::new()
            .data(Client::default())
            .service(web::resource("/something").route(web::post().to(create_something)))
    })
    .bind(endpoint)?
    .run()
    .await
}
