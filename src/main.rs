mod db;
mod features;

use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};
use axum_session::{SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::SessionSqlitePool;
use sqlx::SqlitePool;

use crate::features::user::{handler::{log_out, login, protected, register }, middleware::auth_middleware, model::User};
use crate::db::{init_db, init_session};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = init_db().await?;
    let session_store = init_session(pool.clone()).await?;
    let app = build_app(pool.clone(), session_store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn build_app(pool: SqlitePool, session_store: SessionStore<SessionSqlitePool>) -> Router {
    let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));

    Router::new()
        .route("/", get(|| async { "Hello world!" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(log_out))
        .route(
            "/protected",
            get(protected).route_layer(from_fn(auth_middleware)),
        )
        .layer(
            AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(pool)
}
