use env_logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer, ResponseError};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use derive_more::Display; 


#[derive(Debug, Display)]
pub enum CustomError {
    #[display(fmt = "Custom Error 1")]
    CustomOne,
    #[display(fmt = "Custom Error 2")]
    CustomTwo,
    #[display(fmt = "Custom Error 3")]
    CustomThree,
    #[display(fmt = "Custom Error 4")]
    CustomFour,
}

impl Distribution<CustomError> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0, 4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

async fn do_something_random() -> Result<(), CustomError> {
    let mut rng = thread_rng();

    if rng.gen_bool(2.0/ 10.0) {
        Ok(())
    } else {
        Err(rand::random::<CustomError>())
    }

}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::CustomOne => {
                println!("do some stuff related to CustomOne error");
                HttpResponse::Forbidden().finish()
            }

            CustomError::CustomTwo => {
                println!("do some stuff related to CustomTwo error");
                HttpResponse::Unauthorized().finish()
            }

            CustomError::CustomThree => {
                println!("do some stuff related to CustomThree error");
                HttpResponse::InternalServerError().finish()
            }

            _ => {
                println!("do some stuff related to CustomFour error");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}


async fn do_something() -> Result<HttpResponse, Error> {
    do_something_random().await?;
    Ok(HttpResponse::Ok().body("Nothing Interesting Happened. Try Again"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/something").route(web::get().to(do_something)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
