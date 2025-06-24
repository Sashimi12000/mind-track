//! Daily checkin service - Business logic for daily check-ins

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use chrono::{Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::models::daily_checkins::{self, Entity as DailyCheckins};

/// デイリーチェックイン作成時のペイロード構造体
#[derive(Debug, Deserialize)]
pub struct CreateDailyCheckinPayload {
    pub date: String, // YYYY-MM-DD format
    pub mood_level: i32, // 1-5
    pub mood_text: Option<String>, // mood_memoからmood_textに変更
    pub physical_state_tags: Option<Vec<String>>,
    pub potential_todos: Option<Vec<String>>,
    // feeling_for_todosは削除（エンティティに存在しないため）
}

/// デイリーチェックイン返却用の構造体
#[derive(Debug, Serialize)]
pub struct DailyCheckinResponse {
    pub uuid: String,
    pub date: String,
    pub mood_level: i32,
    pub mood_text: Option<String>, // mood_memoからmood_textに変更
    pub physical_state_tags: Option<Vec<String>>,
    pub potential_todos: Option<Vec<String>>,
    // feeling_for_todosは削除
    pub created_at: String,
    pub updated_at: String,
}

impl From<daily_checkins::Model> for DailyCheckinResponse {
    fn from(model: daily_checkins::Model) -> Self {
        Self {
            uuid: model.uuid,
            date: model.date.to_string(),
            mood_level: model.mood_level,
            mood_text: if model.mood_text.is_empty() { None } else { Some(model.mood_text) },
            physical_state_tags: if model.physical_state_tags.is_empty() {
                None
            } else {
                serde_json::from_str(&model.physical_state_tags).ok()
            },
            potential_todos: if model.potential_todos.is_empty() {
                None
            } else {
                serde_json::from_str(&model.potential_todos).ok()
            },
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

/// デイリーチェックインサービス
pub struct DailyCheckinService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DailyCheckinService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// デイリーチェックインを記録する
    pub async fn record_checkin(
        &self,
        payload: CreateDailyCheckinPayload,
    ) -> Result<DailyCheckinResponse> {
        // バリデーション
        self.validate_payload(&payload)?;

        // 日付の解析
        let date = NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d")
            .map_err(|_| AppError::validation("date", "Invalid date format", None))?;

        // 既存のレコードをチェック（同日の記録があるかどうか）
        let existing = DailyCheckins::find()
            .filter(daily_checkins::Column::Date.eq(date))
            .filter(daily_checkins::Column::DeletedAt.is_null())
            .one(self.db)
            .await
            .map_err(AppError::from)?;

        if existing.is_some() {
            return Err(AppError::validation(
                "date",
                "Daily checkin already exists for this date",
                Some("この日のチェックインは既に記録済みです。"),
            ));
        }

        // JSON文字列にシリアライズ
        let physical_state_tags_json = payload.physical_state_tags
            .map(|tags| serde_json::to_string(&tags))
            .transpose()
            .map_err(|e| AppError::unexpected(&e.to_string(), None))?
            .unwrap_or_else(|| String::new());

        let potential_todos_json = payload.potential_todos
            .map(|todos| serde_json::to_string(&todos))
            .transpose()
            .map_err(|e| AppError::unexpected(&e.to_string(), None))?
            .unwrap_or_else(|| String::new());

        // 新しいレコードを作成
        let now = Utc::now();
        let new_checkin = daily_checkins::ActiveModel {
            uuid: Set(Uuid::new_v4().to_string()),
            date: Set(date),
            mood_level: Set(payload.mood_level),
            mood_text: Set(payload.mood_text.unwrap_or_else(|| String::new())),
            physical_state_tags: Set(physical_state_tags_json),
            physical_state_text: Set(String::new()), // 空文字列で初期化
            potential_todos: Set(potential_todos_json),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            deleted_at: Set(Utc::now().into()), // 削除されていない場合は現在時刻
            ..Default::default()
        };

        // データベースに保存
        let saved_checkin = new_checkin.insert(self.db).await.map_err(AppError::from)?;

        Ok(DailyCheckinResponse::from(saved_checkin))
    }

    /// 指定日のデイリーチェックインを取得する
    pub async fn get_checkin_by_date(&self, date: &str) -> Result<Option<DailyCheckinResponse>> {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| AppError::validation("date", "Invalid date format", None))?;

        let checkin = DailyCheckins::find()
            .filter(daily_checkins::Column::Date.eq(date))
            .filter(daily_checkins::Column::DeletedAt.is_null())
            .one(self.db)
            .await
            .map_err(AppError::from)?;

        Ok(checkin.map(DailyCheckinResponse::from))
    }

    /// ペイロードのバリデーションを行う
    fn validate_payload(&self, payload: &CreateDailyCheckinPayload) -> Result<()> {
        // 気分レベルのバリデーション
        if !(1..=5).contains(&payload.mood_level) {
            return Err(AppError::validation(
                "mood_level",
                "Mood level must be between 1 and 5",
                Some("気分レベルは1から5の間で選択してください。"),
            ));
        }

        // 気分メモの文字数制限
        if let Some(ref memo) = payload.mood_text {
            if memo.len() > 500 {
                return Err(AppError::validation(
                    "mood_text",
                    "Mood text must be 500 characters or less",
                    Some("気分メモは500文字以内で入力してください。"),
                ));
            }
        }

        // 体の状態タグのバリデーション
        if let Some(ref tags) = payload.physical_state_tags {
            if tags.len() > 10 {
                return Err(AppError::validation(
                    "physical_state_tags",
                    "Maximum 10 tags allowed",
                    Some("体の状態タグは最大10個まで選択可能です。"),
                ));
            }
            for tag in tags {
                if tag.len() > 20 {
                    return Err(AppError::validation(
                        "physical_state_tags",
                        "Each tag must be 20 characters or less",
                        Some("各タグは20文字以内で入力してください。"),
                    ));
                }
            }
        }

        // やらなきゃリストのバリデーション
        if let Some(ref todos) = payload.potential_todos {
            if todos.len() > 3 {
                return Err(AppError::validation(
                    "potential_todos",
                    "Maximum 3 todos allowed",
                    Some("「やらなきゃ」は最大3つまで入力可能です。"),
                ));
            }
            for todo in todos {
                if todo.len() > 100 {
                    return Err(AppError::validation(
                        "potential_todos",
                        "Each todo must be 100 characters or less",
                        Some("各項目は100文字以内で入力してください。"),
                    ));
                }
            }
        }

        // 日付のバリデーション（未来の日付でないか）
        let date = NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d")
            .map_err(|_| AppError::validation("date", "Invalid date format", None))?;
        
        let today = Utc::now().date_naive();
        if date > today {
            return Err(AppError::validation(
                "date",
                "Cannot record checkin for future dates",
                Some("未来の日付は記録できません。"),
            ));
        }

        Ok(())
    }
}
