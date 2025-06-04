pub use sea_orm_migration::prelude::*;

mod m20250604_064814_create_table_daily_checkins;
mod m20250604_065127_create_table_daily_micro_tasks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250604_064814_create_table_daily_checkins::Migration),
            Box::new(m20250604_065127_create_table_daily_micro_tasks::Migration),
        ]
    }
}
