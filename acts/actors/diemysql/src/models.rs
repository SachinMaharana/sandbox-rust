// use chrono::DateTime;
// use diesel::sql_types::Datetime;

#[derive(Queryable)]
pub struct Snippet {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created: chrono::NaiveDateTime,
    pub expires: chrono::NaiveDateTime,
}

use super::schema::snippets;
#[derive(Insertable)]
#[table_name = "snippets"]
pub struct NewSnippet {
    // pub id: i32,
    pub title: String,
    pub content: String,
    // pub created: chrono::NaiveDateTime,
    // pub expires: chrono::NaiveDateTime,
}
