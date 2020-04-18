#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::debug_query;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
pub mod models;
use diesel::mysql::Mysql;
pub mod schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}: ", database_url))
}

use self::models::{NewSnippet, Snippet};

pub fn create_snippet(conn: &MysqlConnection, title: String, content: String) -> Snippet {
    use schema::snippets;

    let new_snippet = NewSnippet {
        title: title.to_string(),
        content: content.to_string(),
    };
    let q = diesel::insert_into(snippets::table).values(&new_snippet);
    println!("{}", debug_query::<Mysql, _>(&q).to_string());
    diesel::insert_into(snippets::table)
        .values(&new_snippet)
        // .default_values()
        .execute(conn)
        .expect("Error Saving Post");

    snippets::table
        .order(snippets::id.desc())
        .first(conn)
        .unwrap()
}
