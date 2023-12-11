use crate::get;
use std::sync::Arc;

use rocket::State;
use sqlx::SqlitePool;

use crate::models::{UserService, User};

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }

    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id).fetch_optional(&self.pool).await
    }
}

#[get("/users")]
pub async fn user_handler(user_service: &State<Arc<UserService>>) -> &'static str {
    let user_id:i64 = 1;
    match user_service.fetch_user(user_id).await {
        Ok(_) => println!("success"),
        Err(_) => println!("error"),
    }
    "hoge"
}
