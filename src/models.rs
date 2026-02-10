use serde::Deserialize;
use sqlx::FromRow;

// Structs
#[derive(Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub username: String,
}

#[derive(FromRow)]
pub struct UserSql {
    pub id: i32,
    pub username: String,
    pub password: String,
}
