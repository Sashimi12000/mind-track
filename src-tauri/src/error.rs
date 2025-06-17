use serde::{Serialize, Serializer};
use thiserror::Error;
// use std::fmt; // fmtは現時点では直接使用されていませんが、将来的なカスタマイズのためにコメントアウトで残しても良いでしょう。

// ユーザーに提示するエラーメッセージの分類や、フロントエンドでの処理分岐に利用できるカテゴリ
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum UserMessageKind {
    Database,
    Validation,
    Io,
    NotFound,
    Network,
    Authentication,
    Permission,
    ExternalService,
    IdGeneration,
    TimeUtils,
    // 必要に応じて他のカテゴリを追加
    Unexpected,
}

// アプリケーション全体で使用するエラー型
// #[error(...)] は主に開発者向けのログやデバッグ情報として利用 (Display トレイト経由)
// Serialize トレイトは手動で実装し、フロントエンドに渡す情報を制御する
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error: {details}")]
    Database {
        user_message: String,
        details: String,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },

    #[error("Validation Error on field '{field}': {message}")]
    Validation {
        user_message: String, // ユーザーに示す汎用的なバリデーションエラーメッセージ
        field: String,        // エラーが発生したフィールド名
        message: String,      // 具体的なバリデーション違反の内容 (開発者ログ向け)
    },

    #[error("I/O Error: {details}")]
    Io {
        user_message: String,
        details: String,
        source: std::io::Error, // sourceフィールドは残す
    },

    #[error("Resource Not Found: Type='{resource_type}', ID='{resource_id}'. Details: {details}")]
    NotFound {
        user_message: String,
        resource_type: String,
        resource_id: String,
        details: String,
    },

    #[error("ID Generation Error: {details}")]
    IdGeneration {
        user_message: String,
        details: String,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },

    #[error("Time Utility Error: {details}")]
    TimeUtils {
        user_message: String,
        details: String,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },
    
    #[error("External Service Error ({service_name}): {details}")]
    ExternalService {
        user_message: String,
        service_name: String,
        details: String,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },

    #[error("Unexpected Error: {details}")]
    Unexpected {
        user_message: String,
        details: String,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },
}

// フロントエンドにシリアライズして渡すための構造
// AppError の各バリアントからこの構造にマッピングし、ユーザーに必要な情報のみを選択する
#[derive(Serialize)]
struct SerializableAppError<'a> {
    kind: UserMessageKind, // エラーのカテゴリ
    message: &'a str,      // ユーザーに表示するメッセージ
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<&'a str>, // Validationエラーの場合、関連フィールド
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<&'a str>, // NotFoundエラーの場合
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<&'a str>,   // NotFoundエラーの場合
}

// AppError に Serialize トレイトを実装
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializable = match self {
            AppError::Database { user_message, .. } => SerializableAppError {
                kind: UserMessageKind::Database,
                message: user_message,
                field: None,
                resource_type: None,
                resource_id: None,
            },
            AppError::Validation { user_message, field, .. } => SerializableAppError {
                kind: UserMessageKind::Validation,
                message: user_message,
                field: Some(field),
                resource_type: None,
                resource_id: None,
            },
            AppError::Io { user_message, .. } => SerializableAppError {
                kind: UserMessageKind::Io,
                message: user_message,
                field: None,
                resource_type: None,
                resource_id: None,
            },
            AppError::NotFound { user_message, resource_type, resource_id, .. } => SerializableAppError {
                kind: UserMessageKind::NotFound,
                message: user_message,
                field: None,
                resource_type: Some(resource_type),
                resource_id: Some(resource_id),
            },
            AppError::IdGeneration { user_message, .. } => SerializableAppError {
                kind: UserMessageKind::IdGeneration,
                message: user_message,
                field: None,
                resource_type: None,
                resource_id: None,
            },
            AppError::TimeUtils { user_message, .. } => SerializableAppError {
                kind: UserMessageKind::TimeUtils,
                message: user_message,
                field: None,
                resource_type: None,
                resource_id: None,
            },
            AppError::ExternalService { user_message, .. } => SerializableAppError {
                kind: UserMessageKind::ExternalService,
                message: user_message,
                field: None,
                resource_type: None,
                resource_id: None,
            },
            AppError::Unexpected { user_message, .. } => SerializableAppError {
                kind: UserMessageKind::Unexpected,
                message: user_message,
                field: None,
                resource_type: None,
                resource_id: None,
            },
        };
        serializable.serialize(serializer)
    }
}

// Result 型のエイリアス (このモジュール内で AppError を使う場合に便利)
pub type Result<T, E = AppError> = std::result::Result<T, E>;

// From トレイトの実装 (他のエラー型から AppError への変換)

// sea_orm::DbErr からの変換
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::Database {
            user_message: "データベース処理中にエラーが発生しました。".to_string(),
            details: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

// uuid::Error からの変換 (id_generator.rs で使用想定)
impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::IdGeneration {
            user_message: "IDの生成に失敗しました。".to_string(),
            details: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

// chrono::ParseError からの変換 (time_utils.rs で使用想定)
impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        AppError::TimeUtils {
            user_message: "日時の解析に失敗しました。入力形式を確認してください。".to_string(),
            details: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

// serde_json::Error からの変換 (JSONのパース/シリアライズエラーで使用想定)
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Unexpected { // または専用の AppError::JsonParseError バリアントを作成
            user_message: "データの処理中に問題が発生しました。".to_string(),
            details: format!("JSON processing error: {}", err),
            source: Some(Box::new(err)),
        }
    }
}


// 具体的なエラーを生成するヘルパー関数 (例)
impl AppError {
    pub fn validation(field: &str, message: &str, user_message: Option<&str>) -> Self {
        AppError::Validation {
            user_message: user_message.unwrap_or("入力内容が正しくありません。").to_string(),
            field: field.to_string(),
            message: message.to_string(), // これは開発者向けログ用
        }
    }

    pub fn not_found(resource_type: &str, resource_id: &str, details: &str, user_message: Option<&str>) -> Self {
        AppError::NotFound {
            user_message: user_message.unwrap_or(&format!("指定された{}が見つかりませんでした。", resource_type)).to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.to_string(),
            details: details.to_string(),
        }
    }

    pub fn unexpected(details: &str, user_message: Option<&str>) -> Self {
        AppError::Unexpected {
            user_message: user_message.unwrap_or("予期せぬエラーが発生しました。しばらくしてから再度お試しください。").to_string(),
            details: details.to_string(),
            source: None, // 必要に応じてsourceを設定
        }
    }
}
