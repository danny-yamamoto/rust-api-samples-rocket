use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct User {
    pub user_id: i64,
    pub email_address: Option<String>,
    pub created_at: Option<i64>,
    pub deleted: Option<i64>,
    pub settings: Option<String>,
}

#[derive(Deserialize)]
pub struct UserQuery {
    pub user_id: i64,
}

pub struct UserService {
    pub pool: SqlitePool,
}