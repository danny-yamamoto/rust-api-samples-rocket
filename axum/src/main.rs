use std::{env, sync::Arc};
use axum::{Router, routing::get, Extension};
use dotenv::dotenv;
use models::UserService;
use sqlx::SqlitePool;
mod models;
use routes::user_handler;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "DATABASE_URL".to_string();
    let database_url = env::var(key).expect("Failed to set environment variables");
    let pool = SqlitePool::connect(&database_url).await.expect("Failed to connect database.");
    let user_service = Arc::new(UserService::new(pool));
    let app = Router::new().route("/users/:user_id", get(user_handler)).layer(Extension(user_service));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
