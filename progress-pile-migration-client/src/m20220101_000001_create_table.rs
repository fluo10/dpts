use progress_pile_migration_core::m20220101_000001_create_table::{
    TableMigration,
    ProgressCategory as DefaultProgressCategory,
    ProgressEntry as DefaultProgressEntry,
    PK_PROGRESS_CATEGORY,
    PK_PROGRESS_ENTITY,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ProgressCategory::up(manager).await?;
        ProgressEntry::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ProgressEntry::down(manager).await?;
        ProgressCategory::down(manager).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProgressCategory{}

impl TableMigration for ProgressCategory {
    fn table_create_statement() -> TableCreateStatement{
        let mut tcs = DefaultProgressCategory::table_create_statement();
        tcs.primary_key(Index::create().name(PK_PROGRESS_CATEGORY).col(DefaultProgressCategory::Id));
        tcs
    }
    fn index_create_statements() -> Vec<IndexCreateStatement> {
        DefaultProgressCategory::index_create_statements()
    }
    fn table_drop_statement() -> TableDropStatement{
        DefaultProgressCategory::table_drop_statement()
    }
}


#[derive(DeriveIden)]
pub enum ProgressEntry {}

impl TableMigration for ProgressEntry {

    fn table_create_statement() -> TableCreateStatement{
        let mut tcs: TableCreateStatement = DefaultProgressEntry::table_create_statement();
        tcs.primary_key(Index::create().name(PK_PROGRESS_ENTITY).col(DefaultProgressEntry::Id));
        tcs
    }
    fn index_create_statements() -> Vec<IndexCreateStatement> {
        DefaultProgressEntry::index_create_statements()
    }
    fn table_drop_statement() -> TableDropStatement {
        DefaultProgressEntry::table_drop_statement()
    }
}
