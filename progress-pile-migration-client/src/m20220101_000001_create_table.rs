use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
use progress_pile_migration_core::m20220101_000001_create_table::*;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.create_table(ProgressCategory::client_table_create_statement()).await?;
        manager.create_table(ProgressEntry::client_table_create_statement()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.drop_table(ProgressCategory::table_drop_statement()).await?;
        manager.drop_table(ProgressEntry::table_drop_statement()).await?;
        Ok(())
    }
}
