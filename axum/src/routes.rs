use std::sync::Arc;
use axum::{Extension, extract::Path, http::StatusCode, Json};
use sqlx::SqlitePool;
use crate::models::{UserService, User};

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }
    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id)
            .fetch_optional(&self.pool).await
    }
}

pub async fn user_handler(Path(user_id): Path<i64>, Extension(user_service): Extension<Arc<UserService>>) -> (StatusCode, Json<Option<User>>) {
    match user_service.fetch_user(user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
