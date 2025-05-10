use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
use progress_pile_migration_core::m20220101_000001_create_table::*;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        ProgressCategory::up_client(manager).await?;
        ProgressEntry::up_client(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        ProgressEntry::down_client(manager).await?;
        ProgressCategory::down_client(manager).await?;
        Ok(())
    }
}
