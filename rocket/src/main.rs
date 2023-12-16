use std::{env, sync::Arc};
use dotenv::dotenv;
use models::UserService;
use rocket::{routes, get, launch};
use routes::user_handler;
use sqlx::SqlitePool;

mod models;
mod routes;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let key = "DATABASE_URL".to_string();
    let db_url = env::var(key)
        .expect("Failed to set environment variables.");
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect database.");
    let user_service = Arc::new(UserService::new(pool));
    rocket::build()
        .manage(user_service)
        .mount("/", routes![user_handler])
}
