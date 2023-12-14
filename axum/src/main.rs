use std::{env, sync::Arc};

use axum::{Router, routing::get, Extension, extract::Query, http::StatusCode, Json};
use dotenv::dotenv;
use models::{UserService, UserQuery};
use sqlx::SqlitePool;

use crate::models::User;
mod models;
mod routes;

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }

    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id).fetch_optional(&self.pool).await
    }
}

pub async fn user_handler(Query(query): Query<UserQuery>, Extension(user_service): Extension<Arc<UserService>>) -> (StatusCode, Json<Option<User>>) {
    match user_service.fetch_user(query.user_id).await {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "DATABASE_URL".to_string();
    let database_url = env::var(key).expect("Failed.");
    let pool = SqlitePool::connect(&database_url).await.expect("Failed to connect database.");
    let user_service = Arc::new(UserService::new(pool));
    let app = Router::new().route("/users", get(user_handler)).layer(Extension(user_service));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    //axum::Server::bind(&addr)
    //let app = Router::new();
    // router
    // port
    // ip
    // axum
}
