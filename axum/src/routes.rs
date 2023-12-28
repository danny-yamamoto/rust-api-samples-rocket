use std::sync::Arc;
use axum::{Extension, extract::Path, http::StatusCode, Json, response::IntoResponse};
use cloud_storage::Client;
use sqlx::SqlitePool;
use crate::models::{UserService, User, StorageResponse, StorageQuery};
use axum::response::Response;

pub enum ApiResponse {
    UserResponse(Option<User>),
    StorageResponse(StorageResponse),
    ErrorResponse(String),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            ApiResponse::UserResponse(user) => (StatusCode::OK, Json(user)).into_response(),
            ApiResponse::StorageResponse(storage) => (StatusCode::OK, Json(storage)).into_response(),
            ApiResponse::ErrorResponse(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
        }
    }
}

impl UserService {
    pub fn new(pool: SqlitePool) -> Self {
        UserService { pool }
    }
    pub async fn fetch_user(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT user_id, email_address, created_at, deleted, settings FROM users WHERE user_id = ?", user_id)
            .fetch_optional(&self.pool).await
    }
}

pub async fn user_handler(Path(user_id): Path<i64>, Extension(user_service): Extension<Arc<UserService>>) -> impl IntoResponse {
    match user_service.fetch_user(user_id).await {
        Ok(user) => ApiResponse::UserResponse(user),
        Err(_) => ApiResponse::ErrorResponse("Internal Server Error.".to_string()),
    }
}

pub struct  StorageService;

impl StorageService {
    pub fn new() -> Self {
        StorageService {}
    }

    pub async fn download_content(&self, query: &StorageQuery) -> Result<StorageResponse, String> {
        let client = Client::default();
        match client.object().download(&query.bucket, &query.object).await {
            Ok(bytes) => Ok(StorageResponse { content: String::from_utf8_lossy(&bytes).to_string() }),
            Err(error) => Err(error.to_string()),
        }
    }
}

pub async fn storage_handler(Path(query): Path<StorageQuery>, Extension(storage_service):Extension<Arc<StorageService>>) -> impl IntoResponse {
    match storage_service.download_content(&query).await {
        Ok(content) => ApiResponse::StorageResponse(content),
        Err(error) => ApiResponse::ErrorResponse(error.to_string()),
    }
}