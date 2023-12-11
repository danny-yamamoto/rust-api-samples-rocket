use std::{env, sync::Arc};

use dotenv::dotenv;
use rocket::{routes, get, launch, State};
use serde::Serialize;
use sqlx::SqlitePool;
//#[macro_use] extern crate rocket;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World."
}

#[derive(Serialize)]
pub struct User {
    pub user_id: i64,
    pub email_address: Option<String>,
    pub created_at: Option<i64>,
    pub deleted: Option<i64>,
    pub settings: Option<String>,
}

pub struct UserService {
    pool: SqlitePool,
}

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }

    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id).fetch_optional(&self.pool).await
    }
}

#[get("/users")]
async fn user_handler(user_service: &State<Arc<UserService>>) -> &'static str {
    let user_id:i64 = 1;
    match user_service.fetch_user(user_id).await {
        Ok(_) => println!("success"),
        Err(_) => println!("error"),
    }
    "hoge"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let key = "DATABASE_URL".to_string();
    let db_url = env::var(key).expect("Failed.");
    let pool = SqlitePool::connect(&db_url).await.expect("Failed to connect database.");
    //let user_service = Arc::new(pool);
    let user_service = Arc::new(UserService::new(pool));
    rocket::build()
    .manage(user_service)
    .mount("/", routes![hello_world, user_handler])
}
