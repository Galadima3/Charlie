use sqlx::{Executor, SqlitePool};
use axum_session_sqlx::SessionSqlitePool;
use axum_session::{SessionConfig, SessionStore, Key};
use crate::features::user::model::{User, UserSql};
use async_trait::async_trait;
use axum_session_auth::Authentication;


pub async fn init_db() -> anyhow::Result<SqlitePool> {
    use sqlx::sqlite::SqliteConnectOptions;
    use std::str::FromStr;

    let options = SqliteConnectOptions::from_str("sqlite://db.sqlite")?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL
        )
    "#,
    )
    .await?;

    let guest_exists: Option<(i32,)> = sqlx::query_as("SELECT id FROM users WHERE id = ?1")
        .bind(1)
        .fetch_optional(&pool)
        .await?;

    if guest_exists.is_none() {
        sqlx::query("INSERT INTO users (username, password) VALUES (?1, ?2)")
            .bind("guest")
            .bind("guest")
            .execute(&pool)
            .await?;
    }

    Ok(pool)
}

pub async fn init_session(pool: SqlitePool) -> anyhow::Result<SessionStore<SessionSqlitePool>> {
    let config = SessionConfig::default()
        .with_table_name("session_table")
        .with_key(Key::generate());

    let store = SessionStore::<SessionSqlitePool>::new(Some(pool.into()), config).await?;

    Ok(store)
}

#[async_trait]
impl Authentication<User, i64, SqlitePool> for User {
    async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> anyhow::Result<User> {
        if userid == 1 {
            Ok(User {
                id: userid,
                anonymous: true,
                username: "guest".to_string(),
            })
        } else {
            let user: UserSql = sqlx::query_as("SELECT * FROM users WHERE id = ?1")
                .bind(userid)
                .fetch_one(pool.unwrap())
                .await?;

            Ok(User {
                id: user.id as i64,
                anonymous: false,
                username: user.username,
            })
        }
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }
}
