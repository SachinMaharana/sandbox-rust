use std::time::{Duration,Instant};

use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use env_logger;

mod server;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn chat_route(req: HttpRequest) -> Result<HttpResponse, Error> {
    ws::start()
    unimplemented!()
}


struct WsChatSession {
    id: usize,
    hb: Instant,
    room: String,
    name: Option<String>,
    addr: Addr<server::ChatServer>,
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // let server = server::ChatServer::default().start();

    HttpServer::new(move || {
        App::new()
            // .data()
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "static/websocket.html")
                    .finish()
            })))
            .service(web::resource("/ws/").to(chat_route))
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
