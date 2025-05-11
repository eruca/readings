use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;
use uuid::Uuid;

pub(crate) type AppResult<T> = Result<T, AppError>;

// --- 错误处理 ---
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Item not found: {0}")]
    NotFound(Uuid),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SqlxError(db_err) => {
                error!("Database error: {:?}", db_err); // 详细记录服务器端错误
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal database error occurred".to_string(),
                )
            }
            AppError::ConfigError(msg) => {
                error!("Configuration error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server configuration error".to_string(),
                )
            }
            AppError::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Item with ID {} not found", id),
            ),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}
