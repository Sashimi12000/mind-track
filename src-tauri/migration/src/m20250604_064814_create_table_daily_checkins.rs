use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DailyCheckins::Table)
                    .if_not_exists()
                    .col(pk_auto(DailyCheckins::LocalId)) // 主キーカラム名変更
                    .col(string(DailyCheckins::Uuid).not_null().unique_key()) // Uuid 追加
                    .col(date(DailyCheckins::Date).not_null().unique_key())
                    .col(integer(DailyCheckins::MoodLevel).not_null()) // NOT NULL に変更
                    .col(text(DailyCheckins::MoodText).null()) // 仕様書通り NULL許容, max 500 chars はDBレベルで制約しない
                    .col(text(DailyCheckins::PhysicalStateTags).null()) // 仕様書通り NULL許容
                    .col(text(DailyCheckins::PhysicalStateText).null()) // 仕様書通り NULL許容, max 500 chars はDBレベルで制約しない
                    .col(text(DailyCheckins::PotentialTodos).null()) // 仕様書通り NULL許容
                    .col(timestamp_with_time_zone(DailyCheckins::CreatedAt).not_null()) // CreatedAt 追加
                    .col(timestamp_with_time_zone(DailyCheckins::UpdatedAt).not_null()) // UpdatedAt 追加
                    .col(timestamp_with_time_zone(DailyCheckins::DeletedAt).null()) // DeletedAt 追加
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DailyCheckins::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DailyCheckins {
    Table,
    LocalId, // Id から変更
    Uuid,    // 追加
    Date,
    MoodLevel,
    MoodText,
    PhysicalStateTags,
    PhysicalStateText,
    PotentialTodos,
    CreatedAt, // 追加
    UpdatedAt, // 追加
    DeletedAt, // 追加
}
