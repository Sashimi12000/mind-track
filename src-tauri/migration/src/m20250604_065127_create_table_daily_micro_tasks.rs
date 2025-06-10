use sea_orm_migration::{prelude::*, schema::*};
// DailyCheckins の Iden を参照するために追加
use super::m20250604_064814_create_table_daily_checkins::DailyCheckins;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MicroTasks::Table)
                    .if_not_exists()
                    .col(pk_auto(MicroTasks::LocalId)) // 主キーカラム名変更
                    .col(string(MicroTasks::Uuid).not_null().unique_key()) // Uuid 追加
                    .col(string(MicroTasks::DailyCheckinUuid).not_null()) // daily_checkin_uuid 追加 (型をstringに変更)
                    .col(text(MicroTasks::TaskDescription).not_null()) // max 200 chars はDBレベルで制約しない
                    .col(text(MicroTasks::TaskMemo).null()) // max 1000 chars はDBレベルで制約しない
                    .col(
                        integer(MicroTasks::IsCompleted)
                            .not_null()
                            .default(0),
                    )
                    .col(timestamp_with_time_zone(MicroTasks::CompletedAt).null()) // 型をtimestamp_with_time_zoneに変更
                    .col(timestamp_with_time_zone(MicroTasks::CreatedAt).not_null()) // 型をtimestamp_with_time_zoneに変更
                    .col(timestamp_with_time_zone(MicroTasks::UpdatedAt).not_null()) // UpdatedAt 追加
                    .col(timestamp_with_time_zone(MicroTasks::DeletedAt).null())    // DeletedAt 追加
                    .col(integer(MicroTasks::SortOrder).not_null()) // NOT NULL 制約追加
                    // 外部キー制約
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_micro_tasks_daily_checkin_uuid") // 制約名を変更
                            .from(MicroTasks::Table, MicroTasks::DailyCheckinUuid)
                            .to(DailyCheckins::Table, DailyCheckins::Uuid) // 参照先を DailyCheckins の Uuid に変更
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MicroTasks::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MicroTasks {
    Table,
    LocalId,          // Id から変更
    Uuid,             // 追加
    DailyCheckinUuid, // DailyCheckinId から変更し、型もTEXT(string)に
    TaskDescription,
    TaskMemo,
    IsCompleted,
    CompletedAt,
    CreatedAt,
    UpdatedAt,        // 追加
    DeletedAt,        // 追加
    SortOrder,
}
