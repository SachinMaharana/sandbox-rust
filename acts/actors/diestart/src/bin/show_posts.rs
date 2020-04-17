extern crate diesel;
extern crate diestart;

use self::diesel::prelude::*;
use self::models::*;
use diestart::*;
fn main() {
    use self::schema::posts::dsl::*;
    let connection = establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error Loading Posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
