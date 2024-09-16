// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::generate_context;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::fmt;
use crate::config::dbconnetion::init_db;

mod handler;
mod entity;
mod util;
mod config;


#[tokio::main]
async fn main() {
    // init local db
    let file_appender = rolling::daily("logs", "my_app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);

    // 初始化日志订阅器
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    unsafe {
        tracing_subscriber::fmt()
            .with_writer(non_blocking_appender)
            .with_span_events(FmtSpan::FULL)
            .with_thread_names(true)
            // .pretty()
            .init();
    }

    tracing::info!("初始化数据库......");
    init_db().await;
    tracing::info!("初始化数据库成功......");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::handler::task_cmd::add_task,
            crate::handler::task_cmd::control_task,
            crate::handler::task_cmd::query_task,
            crate::handler::task_cmd::query_task_statics,
            crate::handler::task_cmd::add_tomato_task,
            crate::handler::goals_cmd::add_goals,
            crate::handler::goals_cmd::query_goals,
            crate::handler::goals_cmd::query_goals_detail,
        ])
        .run(generate_context!("tauri.conf.json"))
        .expect("error while running tauri application");
    tracing::info!("启动成功......")
}




