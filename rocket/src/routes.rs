use crate::{get, models::UserService};
use std::sync::Arc;

use rocket::{post, serde::json::Json, State};
use sqlx::SqlitePool;
use rocket::http::{Status, ContentType};

use crate::models::User;

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }

    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id)
            .fetch_optional(&self.pool).await
    }

    pub async fn create_user(&self, user: User) -> Result<Option<User>, sqlx::Error> {
        //let created_at = chrono::Utc::now().timestamp();
        println!("user_id: {}", user.user_id);
        println!("email_address: {:?}", user.email_address);
        println!("settings: {:?}", user.settings);
        let _res = sqlx::query!("INSERT INTO users (user_id, email_address, created_at, deleted, settings) VALUES (?, ?, ?, ?, ?)", user.user_id, "", 0, 0, "").execute(&self.pool).await?;
        Ok(Some(user))
    }
}

#[get("/users/<user_id>")]
pub async fn user_handler(user_id: i64, user_service: &State<Arc<UserService>>) -> (Status, (ContentType, Json<Option<User>>)) {
    match user_service.fetch_user(user_id).await {
        Ok(user) => (Status::Ok, (ContentType::JSON, Json(user))),
        Err(_) => (Status::InternalServerError, (ContentType::JSON, Json(None))),
    }
}

#[post("/users", format = "json", data = "<user_data>")]
pub async fn user_post_handler(user_data: Json<User>, user_service: &State<Arc<UserService>>) -> (Status, (ContentType, Json<Option<User>>)) {
    let user = user_data.into_inner();
    println!("user_id: {}", user.user_id);
    match user_service.create_user(user).await {
        Ok(_) => (Status::Ok, (ContentType::JSON, Json(None))),
        Err(_) => (Status::InternalServerError, (ContentType::JSON, Json(None))),
    }
}
