use sea_orm_migration::{prelude::*, schema::*};


pub trait TableMigration {
    async fn up<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(Self::table_create_statement()).await?;
        for statement in Self::index_create_statements().into_iter() {
            manager.create_index(statement).await?
        }
        Ok(())
    }
    async fn down<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.drop_table(Self::table_drop_statement()).await?;
        Ok(())
    }
    fn table_create_statement() -> TableCreateStatement;
    fn index_create_statements() -> Vec<IndexCreateStatement>;
    fn table_drop_statement() -> TableDropStatement;
}


#[derive(DeriveIden)]
pub enum ProgressCategory {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Name,
}

static IDX_PROGRESS_CATEGORY_NAME: &str = "idx_progress_category_name";
static IDX_PROGRESS_CATEGORY_CREATED_AT: &str = "idx_progress_category_created_at";
static IDX_PROGRESS_CATEGORY_UPDATED_AT: &str = "idx_progress_category_updated_at";
static IDX_PROGRESS_CATEGORY_DELETED_AT: &str = "idx_progress_category_deleted_at";
pub static PK_PROGRESS_CATEGORY: &str = "pk_progress_category";

impl TableMigration for ProgressCategory {
    
    fn table_create_statement() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(uuid(Self::Id))
            .col(string(Self::Name))
            .col(timestamp_with_time_zone(Self::CreatedAt))
            .col(timestamp_with_time_zone(Self::UpdatedAt))
            .col(timestamp_with_time_zone_null(Self::DeletedAt))
            .to_owned()
    }
    fn index_create_statements() -> Vec<IndexCreateStatement> {
        vec![
            Index::create().name(IDX_PROGRESS_CATEGORY_CREATED_AT)
                .table(Self::Table)
                .col(Self::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_DELETED_AT)
                .table(Self::Table)
                .col(Self::DeletedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_NAME)
                .table(Self::Table)
                .col(Self::Name)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UpdatedAt)
                .to_owned(),
        ]
    }
    fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}


#[derive(DeriveIden)]
pub enum ProgressEntry {
    Table,
    Id,
    ProgressCategoryId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    ProgressedAt,
    Quantity,
    Note,
}

static IDX_PROGRESS_ENTITY_CREATED_AT: &str = "idx_progress_entity_created_at";
static IDX_PROGRESS_ENTITY_UPDATED_AT: &str = "idx_progress_entity_updated_at";
static IDX_PROGRESS_ENTITY_DELETED_AT: &str = "idx_progress_entity_deleted_at";
static IDX_PROGRESS_ENTITY_PROGRESSED_AT: &str = "idx_progress_entity_progressed_at";
static FK_PROGRESS_ENTITY_PROGRESS_CATEGORY: &str = "fk_progress_entity_progress_category";
pub static PK_PROGRESS_ENTITY: &str = "pk_progress_entity";

impl TableMigration for ProgressEntry {

    fn table_create_statement() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(uuid(Self::Id))
            .col(uuid(Self::ProgressCategoryId))
            .col(timestamp_with_time_zone(Self::CreatedAt))
            .col(timestamp_with_time_zone(Self::UpdatedAt))
            .col(timestamp_with_time_zone_null(Self::DeletedAt))
            .col(timestamp_with_time_zone(Self::ProgressedAt))
            .col(integer(Self::Quantity))
            .col(string(Self::Note))
            .foreign_key(ForeignKey::create()
                .name(FK_PROGRESS_ENTITY_PROGRESS_CATEGORY)
                .from(Self::Table, Self::ProgressCategoryId)
                .to(ProgressCategory::Table, ProgressCategory::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
            )
            .to_owned()
    }

    fn index_create_statements() -> Vec<IndexCreateStatement> {
        vec![
            Index::create().name(IDX_PROGRESS_ENTITY_CREATED_AT)
                .table(Self::Table)
                .col(Self::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_DELETED_AT)
                .table(Self::Table)
                .col(Self::DeletedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_PROGRESSED_AT)
                .table(Self::Table)
                .col(Self::ProgressedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UpdatedAt)
                .to_owned(),
        ]
    }
    
    fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}
