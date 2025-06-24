//! Database connection management

use sea_orm::{Database, DatabaseConnection, DbErr};

// データベースファイルのパス（プロジェクトルートからの相対パス）
const DATABASE_URL: &str = "sqlite://mind_track.sqlite?mode=rwc";

/// データベース接続を確立する
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    
    // 接続の検証
    db.ping().await?;
    
    Ok(db)
}

/// 型エイリアス：サービス層で使いやすくするため
pub type DbConn = DatabaseConnection;
