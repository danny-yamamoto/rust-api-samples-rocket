use axum::{Router, routing::get};
use dotenv::dotenv;
//use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    dotenv().ok();
    //let key = "DATABASE_URL".to_string();
    //let database_url = env::var(key).expect("Failed.");
    //let pool = SqlitePool::connect(&database_url).await.expect("Failed to connect database.");
    //let shared_pool = Arc::new(pool);
    let app = Router::new().route("/users", get(|| async { "Hello, World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    //axum::Server::bind(&addr)
    //let app = Router::new();
    // router
    // port
    // ip
    // axum
}
