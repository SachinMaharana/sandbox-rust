use bson::doc;
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::error::Error;
use mongodb::Client;
use std::process;

use crate::config::{Config, IConfig};
use crate::models::response::{LoginResponse, Response};
use crate::models::user::{Claims, Login, Register, User};
pub trait IUserRepository {
    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error>;
    fn register(&self, user: Register) -> Response;
    fn login(&self, user: Login) -> Result<LoginResponse, Response>;
}

pub struct UserRepository {
    pub connection: &'static Client,
}

impl IUserRepository for UserRepository {
    fn login(&self, user: Login) -> Result<LoginResponse, Response> {
        match self.find_user_with_email(user.email.to_string()).unwrap() {
            Some(x) => {
                let mut sha = Sha256::new();
                sha.input_str(user.password.as_str());
                if x.password == sha.result_str() {
                    let _config: Config = Config {};
                    let _var;
                    match _config.get_config_with_key("SECRET_KEY") {
                        Some(db) => _var = db,
                        None => {
                            println!("SECRET_KEY not set. Exiting");
                            process::exit(1)
                        }
                    }
                    let key = _var.as_bytes();

                    let mut _date: DateTime<Utc>;
                    if !user.remember_me {
                        _date = Utc::now() + Duration::hours(1);
                    } else {
                        _date = Utc::now() + Duration::days(365);
                    }

                    let my_claims = Claims {
                        sub: user.email,
                        exp: _date.timestamp() as usize,
                    };
                    let token = encode(
                        &Header::default(),
                        &my_claims,
                        &EncodingKey::from_secret(key),
                    )
                    .unwrap();
                    Ok(LoginResponse {
                        status: true,
                        token,
                        message: "You have successfully logged in.".to_string(),
                    })
                } else {
                    Err(Response {
                        status: false,
                        message: "Check your user informations.".to_string(),
                    })
                }
            }
            None => Err(Response {
                status: false,
                message: "Check your user informations.".to_string(),
            }),
        }
    }

    fn register(&self, user: Register) -> Response {
        let users: String = (&user.email).parse().unwrap();
        let exist = self.find_user_with_email(users).unwrap();

        match exist {
            Some(_) => Response {
                message: "This e-mail is using by some user, please enter another e-mail."
                    .to_string(),
                status: false,
            },
            None => {
                let config: Config = Config {};
                let database_name;
                let collection_name;

                match config.get_config_with_key("DATABASE_NAME") {
                    Some(db) => database_name = db,
                    None => {
                        println!("DATABASE_NAME not set. Exiting");
                        process::exit(1)
                    }
                }
                match config.get_config_with_key("USER_COLLECTION_NAME") {
                    Some(val) => collection_name = val,
                    None => {
                        println!("DATABASE_NAME not set. Exiting");
                        process::exit(1)
                    }
                }

                let db = self.connection.database(database_name.as_str());
                let mut sha = Sha256::new();
                sha.input_str(user.password.as_str());
                let hash_pw = sha.result_str();
                let user_id = uuid::Uuid::new_v4().to_string();
                let _ex = db.collection(collection_name.as_str()).insert_one(doc! {"user_id": user_id, "name": user.name, "surname": user.surname, "email": user.email, "password": hash_pw, "phone": "", "birth_date": "" }, None);
                match _ex {
                    Ok(_) => Response {
                        status: true,
                        message: "Register successful.".to_string(),
                    },
                    Err(_) => Response {
                        status: false,
                        message: "Something wrong.".to_string(),
                    },
                }
            }
        }
    }

    fn find_user_with_email(&self, email: String) -> Result<Option<User>, Error> {
        let config: Config = Config {};
        let database_name;
        let collection_name;

        match config.get_config_with_key("DATABASE_NAME") {
            Some(db) => database_name = db,
            None => {
                println!("DATABASE_NAME not set. Exiting");
                process::exit(1)
            }
        }
        match config.get_config_with_key("USER_COLLECTION_NAME") {
            Some(val) => collection_name = val,
            None => {
                println!("DATABASE_NAME not set. Exiting");
                process::exit(1)
            }
        }

        let db = self.connection.database(database_name.as_str());
        let cursor = db
            .collection(collection_name.as_str())
            .find_one(doc! {"email": email}, None)
            .unwrap();

        match cursor {
            Some(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(model) => Ok(model),
                Err(e) => Err(Error::from(e)),
            },
            None => Ok(None),
        }
    }
}
