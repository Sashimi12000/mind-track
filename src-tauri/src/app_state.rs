//! Application state management for Tauri

use crate::db::DbConn;

/// アプリケーション全体で共有される状態
#[derive(Debug)]
pub struct AppState {
    /// データベース接続
    pub db: DbConn,
}

impl AppState {
    /// 新しいAppStateインスタンスを作成
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}
