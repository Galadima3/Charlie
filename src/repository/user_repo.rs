use crate::models::{UserSql};
use sqlx::{SqlitePool};

pub async fn user_exists(pool: &SqlitePool, username: &str) -> bool {
    sqlx::query("SELECT 1 FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
        .unwrap()
        .is_some()
}

pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Option<UserSql> {
    sqlx::query_as::<_, UserSql>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}