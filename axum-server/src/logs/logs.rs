use std::{env, io};

use tracing::level_filters::LevelFilter;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    EnvFilter, Layer, Registry,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

// 从环境变量中读取是否log_to_console和log_to_file
fn tracing_targets_from_env() -> (bool, bool) {
    let log_to_file = env::var("LOG_TO_FILE").unwrap_or("false".to_string());
    let log_to_file_bool = log_to_file.parse().unwrap_or(false);

    let log_to_console = env::var("LOG_TO_CONSOLE").unwrap_or("true".to_string());
    let log_to_console_bool = log_to_console.parse().unwrap_or(true);

    return (log_to_console_bool, log_to_file_bool);
}

pub(crate) fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let (enable_console_logging, enable_file_logging) = tracing_targets_from_env();
    if !enable_console_logging && !enable_file_logging {
        return Ok(());
    }

    // --- 从环境变量配置全局日志级别 ---
    // 例如: RUST_LOG=info
    // 或者 RUST_LOG=axum_postgres_prod=debug,tower_http=info
    // 如果未设置 RUST_LOG，则默认为 "info"
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let log_directory = "logs";
    let max_rotate_files = 15;

    // --- 构建可选的文件日志 Layers ---
    let error_file_layer_opt = if enable_file_logging {
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("error")
            .filename_suffix("json")
            .max_log_files(max_rotate_files)
            .build(log_directory)?;

        Some(
            fmt::layer()
                .with_writer(appender)
                .with_ansi(false)
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_filter(LevelFilter::ERROR),
        )
    } else {
        None
    };

    let warn_file_layer_opt = if enable_file_logging {
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("warn")
            .filename_suffix("json")
            .max_log_files(max_rotate_files)
            .build(log_directory)?;
        Some(
            fmt::layer()
                .with_writer(appender)
                .with_ansi(false)
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_filter(LevelFilter::WARN),
        )
    } else {
        None
    };

    let info_file_layer_opt = if enable_file_logging {
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("info")
            .filename_suffix("json")
            .max_log_files(max_rotate_files)
            .build(log_directory)?;
        
        Some(
            fmt::layer()
                .with_writer(appender)
                .with_ansi(false)
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_filter(LevelFilter::INFO),
        )
    } else {
        None
    };

    let debug_file_layer_opt = if enable_file_logging {
        let appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("debug")
            .filename_suffix("json")
            .max_log_files(max_rotate_files)
            .build(log_directory)?;

        Some(
            fmt::layer()
                .with_writer(appender)
                .with_ansi(false)
                .json()
                .with_current_span(true)
                .with_span_list(true)
                .with_filter(LevelFilter::DEBUG),
        )
    } else {
        None
    };

    // --- 构建可选的控制台日志 Layer ---
    let console_layer_opt = if enable_console_logging {
        Some(
            fmt::layer()
                .with_writer(io::stdout)
                .with_ansi(true)
                .with_span_events(FmtSpan::CLOSE)
                .pretty(),
        )
    } else {
        None
    };

    // --- 组合所有 Layers ---
    Registry::default()
        .with(env_filter) // EnvFilter 是第一个，总是添加
        .with(error_file_layer_opt) // 如果是 Some，则添加；如果是 None，则跳过
        .with(warn_file_layer_opt)
        .with(info_file_layer_opt)
        .with(debug_file_layer_opt)
        .with(console_layer_opt)
        .try_init()?; // 初始化最终的 subscriber

    Ok(())
}
