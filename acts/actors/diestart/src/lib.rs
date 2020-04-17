#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewPost, Post};
use diesel::debug_query;
use diesel::pg::Pg;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;
    use schema::posts::columns;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    let r = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post");
    // println!("{:?}", debug_query::<Pg, _>(&r));
    r
}
