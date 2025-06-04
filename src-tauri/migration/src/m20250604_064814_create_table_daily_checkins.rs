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
                    .col(pk_auto(DailyCheckins::Id))
                    .col(date(DailyCheckins::Date).not_null().unique_key())
                    .col(integer(DailyCheckins::MoodLevel).null())
                    .col(text(DailyCheckins::MoodText).null())
                    .col(text(DailyCheckins::PhysicalStateTags).null())
                    .col(text(DailyCheckins::PhysicalStateText).null())
                    .col(text(DailyCheckins::PotentialTodos).null())
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
enum DailyCheckins {
    Table,
    Id,
    Date,
    MoodLevel,
    MoodText,
    PhysicalStateTags,
    PhysicalStateText,
    PotentialTodos,
}
