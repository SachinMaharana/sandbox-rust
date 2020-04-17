extern crate diemysql;
extern crate diesel;

use self::diemysql::*;
use self::diesel::prelude::*;
use self::models::*;

fn main() {
    use diemysql::schema::snippets::dsl::*;

    let conn = establish_connection();
    let result = snippets
        .load::<Snippet>(&conn)
        .expect("Error Loading Posts");

    println!("Displaying {} posts", result.len());
    for post in result {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.content);
        println!("{}", post.created);
        println!("{}", post.expires);
    }
}
