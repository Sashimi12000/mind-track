pub mod app_state;
pub mod commands;
pub mod db;
pub mod entity;
pub mod error;
pub mod models;
pub mod services;
pub mod utils;

use crate::{app_state::AppState, db::establish_connection};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // データベース接続を初期化
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    let db = rt.block_on(async {
        establish_connection().await.expect("Failed to establish database connection")
    });

    // アプリケーション状態を作成
    let app_state = AppState::new(db);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)        .invoke_handler(tauri::generate_handler![
            greet,
            commands::record_daily_checkin,
            commands::get_daily_checkin_by_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}