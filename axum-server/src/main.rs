use std::{env, net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use sqlx::PgPool;
use tracing::info;

mod errors;
mod logs;
mod models;
mod utils;

use errors::AppError;
use logs::setup_tracing;
use models::items::create_router;
use utils::{health_check, initialize_postgresql, shutdown_signal};

// --- 应用程序状态 ---
// 使用 Arc 来安全地在多线程间共享 PgPool
type AppState = Arc<PgPool>;

// --- 主函数 ---
#[tokio::main]
async fn main() -> Result<(), AppError> {
    // 1. 配置管理: 从 .env 文件加载环境变量
    dotenvy::dotenv().ok(); // 如果 .env 不存在也不会报错

    // 2. 日志记录: 初始化 tracing
    // RUST_LOG 环境变量用于控制日志级别 (例如: info,axum_postgres_prod=debug)
    if let Err(e) = setup_tracing() {
        eprintln!("Failed to set up tracing: {}", e);
        // 根据需要决定是否在此处 panic 或退出
        // 对于生产应用，通常会记录这个错误到某个备用机制然后尝试继续或安全退出
        return Err(AppError::ConfigError(format!(
            "Tracing setup failed: {}",
            e
        )));
    }

    info!("Tracing initialized...\nStarting server...");

    // 运行数据库迁移 (生产环境中通常在部署脚本中执行)
    // sqlx::migrate!("./migrations").run(&pool).await.map_err(AppError::SqlxError)?;
    // info!("Database migrations applied.");
    // 注意: 你需要创建 migrations 文件夹并使用 sqlx-cli 来创建迁移文件
    // 例如: sqlx migrate add create_items_table
    let pool = initialize_postgresql(10).await?;
    let app_state = Arc::new(pool);

    // 4. 定义路由
    let app = Router::new()
        .merge(create_router())
        .route("/health", get(health_check)) // 健康检查端点
        .with_state(app_state);

    // 5. 启动服务器
    let server_addr_str = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let addr: SocketAddr = server_addr_str
        .parse()
        .map_err(|_| AppError::ConfigError(format!("Invalid SERVER_ADDR: {}", server_addr_str)))?;

    info!("Listening on {}", addr);

    // 6. 优雅停机 (Axum 0.6+ 内置支持)
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("Server shut down gracefully.");
    Ok(())
}
