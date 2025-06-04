use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(pk_auto(MicroTasks::Id))
                    .col(integer(MicroTasks::DailyCheckinId).null()) // 仕様書通りNULL許容
                    .col(text(MicroTasks::TaskDescription).not_null())
                    .col(text(MicroTasks::TaskMemo).null())
                    .col(
                        integer(MicroTasks::IsCompleted)
                            .not_null()
                            .default(0),
                    )
                    .col(timestamp(MicroTasks::CompletedAt).null())
                    .col(
                        timestamp(MicroTasks::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(MicroTasks::SortOrder).null()) // 初期リリースでは使用しないがカラムは用意
                    // 外部キー制約
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_micro_tasks_daily_checkin_id")
                            .from(MicroTasks::Table, MicroTasks::DailyCheckinId)
                            .to(DailyCheckins::Table, DailyCheckins::Id) // 前のマイグレーションで作成したテーブルを参照
                            .on_delete(ForeignKeyAction::Cascade) // 親レコード削除時に子レコードも削除 (またはSetNullなど)
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
    Id,
    DailyCheckinId,
    TaskDescription,
    TaskMemo,
    IsCompleted,
    CompletedAt,
    CreatedAt,
    SortOrder,
}

// 外部キー制約のために DailyCheckins の Iden を参照できるようにする
// 本来は別ファイルから import するか、共通の場所に定義するのが望ましい
#[derive(DeriveIden)]
enum DailyCheckins {
    Table,
    Id,
}
