//! Daily checkin Tauri commands

use tauri::State;

use crate::app_state::AppState;
use crate::error::Result;
use crate::services::daily_checkin_service::{
    CreateDailyCheckinPayload, DailyCheckinResponse, DailyCheckinService,
};

/// デイリーチェックインを記録する
#[tauri::command]
pub async fn record_daily_checkin(
    payload: CreateDailyCheckinPayload,
    state: State<'_, AppState>,
) -> Result<DailyCheckinResponse> {
    let service = DailyCheckinService::new(&state.db);
    service.record_checkin(payload).await
}

/// 指定日のデイリーチェックインを取得する
#[tauri::command]
pub async fn get_daily_checkin_by_date(
    date: String,
    state: State<'_, AppState>,
) -> Result<Option<DailyCheckinResponse>> {
    let service = DailyCheckinService::new(&state.db);
    service.get_checkin_by_date(&date).await
}
