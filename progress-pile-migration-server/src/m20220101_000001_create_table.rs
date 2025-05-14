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
        User::up(manager).await?;
        AccessToken::up(manager).await?;
        ProgressCategory::up(manager).await?;
        ProgressEntry::up(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ProgressEntry::down(manager).await?;
        ProgressCategory::down(manager).await?;
        AccessToken::down(manager).await?;
        User::down(manager).await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    LoginName,
    PasswordHash,
}

static IDX_USER_LOGIN_NAME: &str = "idx_user_login_name";
static IDX_USER_CREATED_AT: &str = "idx_user_created_at";
static IDX_USER_UPDATED_AT: &str = "idx_user_updated_at";
static IDX_USER_DELETED_AT: &str = "idx_user_deleted_at";

impl TableMigration for User {
    
    fn table_create_statement() -> TableCreateStatement{
        Table::create()
        .table(Self::Table)
        .if_not_exists()
        .col(pk_auto(Self::Id))
        .col(timestamp_with_time_zone(Self::CreatedAt))
        .col(timestamp_with_time_zone(Self::UpdatedAt))
        .col(timestamp_with_time_zone_null(Self::DeletedAt))
        .col(string_uniq(Self::LoginName))
        .col(string(Self::PasswordHash))
        
        .to_owned()
    }

    fn index_create_statements() -> Vec<IndexCreateStatement> {
        vec![
            Index::create().name(IDX_USER_LOGIN_NAME)
                .table(Self::Table)
                .col(Self::LoginName)
                .to_owned(),
            Index::create().name(IDX_USER_CREATED_AT)
                .table(Self::Table)
                .col((Self::CreatedAt, IndexOrder::Desc))
                .to_owned(),
            Index::create().name(IDX_USER_UPDATED_AT)
                .table(Self::Table)
                .col((Self::UpdatedAt, IndexOrder::Desc))
                .to_owned(),
            Index::create().name(IDX_USER_DELETED_AT)
                .table(Self::Table)
                .col((Self::DeletedAt, IndexOrder::Desc))
                .to_owned(),
        ]
    }

    fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[derive(DeriveIden)]
pub enum AccessToken{
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    ExpiredAt,
    TokenValue,
    Note,
}

static IDX_ACCESS_TOKEN_TOKEN_VALUE: &str = "idx_access_token_token_value";
static IDX_ACCESS_TOKEN_CREATED_AT: &str = "idx_access_token_created_at";
static IDX_ACCESS_TOKEN_UPDATED_AT: &str = "idx_access_token_updated_at";
static IDX_ACCESS_TOKEN_EXPIRED_AT: &str = "idx_access_token_expired_at";
static IDX_ACCESS_TOKEN_USER_ID_CREATED_AT: &str = "idx_access_token_user_id_created_at";
static IDX_ACCESS_TOKEN_USER_ID_UPDATED_AT: &str = "idx_access_token_user_id_updated_at";
static IDX_ACCESS_TOKEN_USER_ID_EXPIRED_AT: &str = "idx_access_token_user_id_expired_at";
static FK_ACCESS_TOKEN_USER: &str = "fk_access_token_user";

impl TableMigration for AccessToken {

    fn table_create_statement() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(pk_auto(Self::Id))
            .col(integer(Self::UserId))
            .col(timestamp_with_time_zone(Self::CreatedAt))
            .col(timestamp_with_time_zone(Self::UpdatedAt))
            .col(timestamp_with_time_zone_null(Self::ExpiredAt))
            .col(string(Self::TokenValue))
            .col(string(Self::Note))
            .foreign_key(ForeignKey::create()
                .name(FK_ACCESS_TOKEN_USER)
                .from(Self::Table, Self::UserId)
                .to(User::Table, User::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
            )
            .to_owned()
    }
    fn index_create_statements() -> Vec<IndexCreateStatement> {
        vec![
            Index::create().name(IDX_ACCESS_TOKEN_CREATED_AT)
                .table(Self::Table)
                .col(Self::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_ACCESS_TOKEN_EXPIRED_AT)
                .table(Self::Table)
                .col(Self::ExpiredAt)
                .to_owned(),
            Index::create().name(IDX_ACCESS_TOKEN_TOKEN_VALUE)
                .table(Self::Table)
                .col(Self::TokenValue)
                .to_owned(),
            Index::create().name(IDX_ACCESS_TOKEN_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UpdatedAt)
                .to_owned(),
            Index::create().name(IDX_ACCESS_TOKEN_USER_ID_CREATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_ACCESS_TOKEN_USER_ID_EXPIRED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::ExpiredAt)
                .to_owned(),
                Index::create().name(IDX_ACCESS_TOKEN_USER_ID_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::UpdatedAt)
                .to_owned(),            
        ]
    }
    fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[derive(DeriveIden)]
pub enum ProgressCategory {
    Table,
    UserId,
}

static IDX_PROGRESS_CATEGORY_USER_ID_NAME: &str = "idx_progress_category_user_id_name";
static IDX_PROGRESS_CATEGORY_USER_ID_CREATED_AT: &str = "idx_progress_category_user_id_created_at";
static IDX_PROGRESS_CATEGORY_USER_ID_UPDATED_AT: &str = "idx_progress_category_user_id_updated_at";
static IDX_PROGRESS_CATEGORY_USER_ID_DELETED_AT: &str = "idx_progress_category_user_id_deleted_at";
static FK_PROGRESS_CATEGORY_USER: &str = "fk_progress_category_user";

impl TableMigration for ProgressCategory {

    fn table_create_statement() -> TableCreateStatement{
        let mut tcs = DefaultProgressCategory::table_create_statement();
        tcs.col(integer(Self::UserId));
        tcs.foreign_key(ForeignKey::create().name(FK_PROGRESS_CATEGORY_USER).from(Self::Table, Self::UserId)
            .to(User::Table, User::Id));
        tcs.primary_key(Index::create().name(PK_PROGRESS_CATEGORY).col(Self::UserId).col(DefaultProgressCategory::Id));
        tcs
    }
    
    fn index_create_statements() -> Vec<IndexCreateStatement> {
        [DefaultProgressCategory::index_create_statements(), vec![
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_CREATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressCategory::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_DELETED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressCategory::DeletedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_NAME)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressCategory::Name)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressCategory::UpdatedAt)
                .to_owned(),            
        ]].concat()
    }
    fn table_drop_statement() -> TableDropStatement {
        DefaultProgressCategory::table_drop_statement()
    }


}

#[derive(DeriveIden)]
pub enum ProgressEntry {
    Table,
    UserId,
}

static IDX_PROGRESS_ENTITY_USER_ID_CREATED_AT: &str = "idx_progress_entity_user_id_created_at";
static IDX_PROGRESS_ENTITY_USER_ID_UPDATED_AT: &str = "idx_progress_entity_user_id_updated_at";
static IDX_PROGRESS_ENTITY_USER_ID_DELETED_AT: &str = "idx_progress_entity_user_id_deleted_at";
static IDX_PROGRESS_ENTITY_USER_ID_PROGRESSED_AT: &str = "idx_progress_entity_user_id_progressed_at";
static FK_PROGRESS_ENTITY_PROGRESS_CATEGORY: &str = "fk_progress_entity_progress_category";
static FK_PROGRESS_ENTITY_USER: &str = "fk_progress_entity_user";

impl TableMigration for ProgressEntry {

    fn table_create_statement() -> TableCreateStatement{
        let mut tcs: TableCreateStatement = DefaultProgressEntry::table_create_statement();
        tcs.col(integer(Self::UserId));
        tcs.foreign_key(ForeignKey::create()
            .name(FK_PROGRESS_ENTITY_USER)
            .from(Self::Table, Self::UserId)
            .to(User::Table, User::Id)
        );
        tcs.primary_key(Index::create().name(PK_PROGRESS_ENTITY).col(Self::UserId).col(DefaultProgressEntry::Id));

        tcs

    }

    fn index_create_statements() -> Vec<IndexCreateStatement> {
        let mut default = DefaultProgressEntry::index_create_statements();
        default.append(&mut vec![
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_CREATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressEntry::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_DELETED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressEntry::DeletedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_PROGRESSED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressEntry::ProgressedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(DefaultProgressEntry::UpdatedAt) 
                .to_owned(),           
        ]);
        default
    }
    
    fn table_drop_statement() -> TableDropStatement {
        DefaultProgressEntry::table_drop_statement()
    }


}
