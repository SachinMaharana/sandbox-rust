use crate::db::db::{Connection, IConnection};
use crate::models::user::{Login, Register};
use crate::repositories::user_repository::{IUserRepository, UserRepository};
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(health);
    // cfg.service(user_information_get);
    // cfg.service(protected);
}

#[post("/login")]
async fn login(_user: web::Json<Login>) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _respository: UserRepository = UserRepository {
        connection: _connection.init(),
    };

    let proc = _respository.login(_user.into_inner());
    match proc {
        Ok(_) => HttpResponse::Ok().json(proc.unwrap()),
        Err(_) => HttpResponse::Ok()
            .status(StatusCode::from_u16(401).unwrap())
            .json(proc.unwrap_err()),
    }
}
#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json("Okay")
}

#[post("/register")]
async fn register(user: web::Json<Register>) -> HttpResponse {
    let _connection: Connection = Connection {};

    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };

    HttpResponse::Ok().json(_repository.register(user.into_inner()))
}
