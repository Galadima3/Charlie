use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionSqlitePool;
use sqlx::SqlitePool;
use crate::features::user::model::User;

pub async fn auth_middleware(
    auth: AuthSession<User, i64, SessionSqlitePool, SqlitePool>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    if auth.is_authenticated() {
        if let Some(user) = auth.current_user.clone() {
            req.extensions_mut().insert(user);
        }
        next.run(req).await
    } else {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}
