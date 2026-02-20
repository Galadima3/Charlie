
use sqlx::SqlitePool;

use crate::{models::UserSql, repository::user_repo};
#[derive(Debug)]
pub enum UserServiceError {
    UserAlreadyExists,
    UserNotFound,
}

// check user
pub async fn check_user_exists(pool: &SqlitePool, username: &str) -> Result<(), UserServiceError>{
    let exists = user_repo::user_exists(pool, username).await;
    if exists {
        return Err(UserServiceError::UserAlreadyExists);
    }

    Ok(())
} 

// get user
pub async fn get_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<UserSql, UserServiceError> {
    match user_repo::get_user_by_username(pool, username).await {
        Some(user) => Ok(user),
        None => Err(UserServiceError::UserNotFound),
    }
}