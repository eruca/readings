use std::env;

use axum::{Json, http::StatusCode, response::IntoResponse};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::signal::{
    ctrl_c,
    unix::{SignalKind, signal},
};
use tracing::{debug, info, instrument};

use crate::errors::{AppError, AppResult};

pub async fn shutdown_signal() {
    let ctrl_c = async {
        ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {info!("Received Ctrl+C, shutting down...")},
        _ = terminate => {info!("Received terminate signal, shutting down...")},
    }
}

#[instrument]
pub async fn health_check() -> impl IntoResponse {
    debug!("Health check requested");
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

// 数据库连接池
// {max}: 最大连接数 = 10
#[instrument]
pub(crate) async fn initialize_postgresql(max: u32) -> AppResult<PgPool> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| AppError::ConfigError("DATABASE_URL must be set".to_string()))?;

    let pool = PgPoolOptions::new()
        .max_connections(max) // 根据需要调整
        .connect(&database_url)
        .await?;
    // .map_err(AppError::SqlxError)?;

    info!("Database pool initialized.");

    Ok(pool)
}
