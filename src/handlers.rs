use crate::db::{get_user_by_username, user_exists};
use crate::models::{User, UserRequest};
use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionSqlitePool;
use bcrypt::{hash, verify};
use sqlx::SqlitePool;

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<UserRequest>,
) -> impl IntoResponse {
    if user_exists(&pool, &payload.username).await {
        return (
            StatusCode::BAD_REQUEST,
            format!("Username '{}' is already taken", payload.username),
        )
            .into_response();
    }

    let hashed_password = hash(&payload.password, 10).unwrap_or_default();

    if let Err(_) = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&payload.username)
        .bind(&hashed_password)
        .execute(&pool)
        .await
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
    }

    StatusCode::CREATED.into_response()
}

pub async fn login(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UserRequest>,
) -> impl IntoResponse {
    let user = match get_user_by_username(&pool, &payload.username).await {
        Some(u) => u,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Username '{}' is not registered", payload.username),
            )
                .into_response();
        }
    };

    if verify(&payload.password, &user.password).unwrap_or(false) {
        auth.login_user(user.id as i64);
        (StatusCode::OK, "Login successful").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Incorrect password").into_response()
    }
}

pub async fn log_out(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
) -> impl IntoResponse {
    auth.logout_user();
    (StatusCode::OK, "Logged out successfully").into_response()
}

pub async fn protected(Extension(user): Extension<User>) -> impl IntoResponse {
    (
        StatusCode::OK,
        format!("Hello {}, your id is {}", user.username, user.id),
    )
}
