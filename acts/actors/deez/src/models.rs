use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Insertable, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewUser {
    pub name: String,
}
