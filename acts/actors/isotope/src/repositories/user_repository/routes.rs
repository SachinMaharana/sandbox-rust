use crate::db::db::{Connection, IConnection};
use crate::middlewares::auth::AuthorizationService;
use crate::models::user::{Login, Register};
use crate::repositories::user_repository::{IUserRepository, UserRepository};
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(health);
    cfg.service(user_information);
    cfg.service(protected);
}

#[get("/protectedRoute")]
async fn protected(_: AuthorizationService) -> HttpResponse {
    let _connection: Connection = Connection {};
    let _repository: UserRepository = UserRepository {
        connection: _connection.init(),
    };
    HttpResponse::Ok().json(_repository.protected_function())
}

#[post("/login")]
async fn login(user: web::Json<Login>) -> HttpResponse {
    let connection: Connection = Connection {};
    let respository: UserRepository = UserRepository {
        connection: connection.init(),
    };

    let proc = respository.login(user.into_inner());
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
    let connection: Connection = Connection {};

    let repository: UserRepository = UserRepository {
        connection: connection.init(),
    };

    HttpResponse::Ok().json(repository.register(user.into_inner()))
}

#[get("/userInformation")]
async fn user_information(req: HttpRequest) -> HttpResponse {
    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();

    let token = split[1].trim();

    let connection = Connection {};

    let repository = UserRepository {
        connection: connection.init(),
    };

    match repository.user_information(token) {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(err) => HttpResponse::Ok().json(err),
    }
}
