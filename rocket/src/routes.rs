use crate::get;
use std::sync::Arc;

use rocket::{State, serde::json::Json};
use sqlx::SqlitePool;
use rocket::http::{Status, ContentType};

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

#[get("/users/<user_id>")]
pub async fn user_handler(user_id: i64, user_service: &State<Arc<UserService>>) -> (Status, (ContentType, Json<Option<User>>)) {
    match user_service.fetch_user(user_id).await {
        Ok(user) => (Status::Ok, (ContentType::JSON, Json(user))),
        Err(_) => (Status::InternalServerError, (ContentType::JSON, Json(None))),
    }
}
