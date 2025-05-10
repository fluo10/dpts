use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct ClientMigration;

#[async_trait::async_trait]
impl MigrationTrait for ClientMigration {
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

#[cfg(feature="server")]
#[derive(DeriveMigrationName)]
pub struct ServerMigration;

#[cfg(feature="server")]

#[async_trait::async_trait]
impl MigrationTrait for ServerMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        User::up_server(manager).await?;
        AccessToken::up_server(manager).await?;
        ProgressCategory::up_server(manager).await?;
        ProgressEntry::up_server(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        ProgressEntry::down_server(manager).await?;
        ProgressCategory::down_server(manager).await?;
        AccessToken::down_server(manager).await?;
        User::down_server(manager).await
    }
}




pub trait MigrationTableDefault {

    fn table_create_statement_default() -> TableCreateStatement;
    fn index_create_statements_default() -> Vec<IndexCreateStatement>;
    fn table_drop_statement_default() -> TableDropStatement;

}





#[cfg(feature="client")]
#[async_trait::async_trait]
pub trait MigrationTableClient {
    async fn up_client<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(Self::table_create_statement_client()).await?;
        for statement in Self::index_create_statements_client().into_iter() {
            manager.create_index(statement).await?
        }
        Ok(())
    }
    async fn down_client<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.drop_table(Self::table_drop_statement_client()).await?;
        Ok(())
    }
    fn table_create_statement_client() -> TableCreateStatement;
    fn index_create_statements_client() -> Vec<IndexCreateStatement>;
    fn table_drop_statement_client() -> TableDropStatement;
}


#[cfg(feature="server")]
#[async_trait::async_trait]
pub trait MigrationTableServer {

    async fn up_server<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.create_table(Self::table_create_statement_server()).await?;
        for statement in Self::index_create_statements_server().into_iter() {
            manager.create_index(statement).await?
        }
        Ok(())
    }

    async fn down_server<'a>(manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        manager.drop_table(Self::table_drop_statement_server()).await?;
        Ok(())
    }
    
    fn table_create_statement_server() -> TableCreateStatement;
    fn index_create_statements_server() -> Vec<IndexCreateStatement>;
    fn table_drop_statement_server() -> TableDropStatement;
}

#[cfg(feature="server")]
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

#[cfg(feature="server")]
static IDX_USER_LOGIN_NAME: &str = "idx_user_login_name";
#[cfg(feature="server")]
static IDX_USER_CREATED_AT: &str = "idx_user_created_at";
#[cfg(feature="server")]
static IDX_USER_UPDATED_AT: &str = "idx_user_updated_at";
#[cfg(feature="server")]
static IDX_USER_DELETED_AT: &str = "idx_user_deleted_at";

#[cfg(feature="server")]
impl MigrationTableServer for User {
    
    fn table_create_statement_server() -> TableCreateStatement{
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

    fn index_create_statements_server() -> Vec<IndexCreateStatement> {
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

    fn table_drop_statement_server() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[cfg(feature="server")]
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

#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_TOKEN_VALUE: &str = "idx_access_token_token_value";
#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_CREATED_AT: &str = "idx_access_token_created_at";
#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_UPDATED_AT: &str = "idx_access_token_updated_at";
#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_EXPIRED_AT: &str = "idx_access_token_expired_at";
#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_USER_ID_CREATED_AT: &str = "idx_access_token_user_id_created_at";
#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_USER_ID_UPDATED_AT: &str = "idx_access_token_user_id_updated_at";
#[cfg(feature="server")]
static IDX_ACCESS_TOKEN_USER_ID_EXPIRED_AT: &str = "idx_access_token_user_id_expired_at";
#[cfg(feature="server")]
static FK_ACCESS_TOKEN_USER: &str = "fk_access_token_user";

#[cfg(feature="server")]
impl MigrationTableServer for AccessToken {

    fn table_create_statement_server() -> TableCreateStatement {
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
    fn index_create_statements_server() -> Vec<IndexCreateStatement> {
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
    fn table_drop_statement_server() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[derive(DeriveIden)]
pub enum ProgressCategory {
    Table,
    #[cfg(feature="server")]
    UserId,
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
#[cfg(feature="server")]
static IDX_PROGRESS_CATEGORY_USER_ID_NAME: &str = "idx_progress_category_user_id_name";
#[cfg(feature="server")]
static IDX_PROGRESS_CATEGORY_USER_ID_CREATED_AT: &str = "idx_progress_category_user_id_created_at";
#[cfg(feature="server")]
static IDX_PROGRESS_CATEGORY_USER_ID_UPDATED_AT: &str = "idx_progress_category_user_id_updated_at";
#[cfg(feature="server")]
static IDX_PROGRESS_CATEGORY_USER_ID_DELETED_AT: &str = "idx_progress_category_user_id_deleted_at";
#[cfg(feature="server")]
static FK_PROGRESS_CATEGORY_USER: &str = "fk_progress_category_user";
static PK_PROGRESS_CATEGORY: &str = "pk_progress_category";

impl MigrationTableDefault for ProgressCategory {
    
    fn table_create_statement_default() -> TableCreateStatement {
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
    fn index_create_statements_default() -> Vec<IndexCreateStatement> {
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
    fn table_drop_statement_default() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[cfg(feature="client")]
impl MigrationTableClient for ProgressCategory {
    fn table_create_statement_client() -> TableCreateStatement{
        let mut tcs = Self::table_create_statement_default();
        tcs.primary_key(Index::create().name(PK_PROGRESS_CATEGORY).col(Self::Id));
        tcs
    }
    fn index_create_statements_client() -> Vec<IndexCreateStatement> {
        Self::index_create_statements_default()
    }
    fn table_drop_statement_client() -> TableDropStatement{
        Self::table_drop_statement_default()
    }
}

#[cfg(feature="server")]
impl MigrationTableServer for ProgressCategory {

    fn table_create_statement_server() -> TableCreateStatement{
        let mut tcs = Self::table_create_statement_default();
        tcs.col(integer(Self::UserId));
        tcs.foreign_key(ForeignKey::create().name(FK_PROGRESS_CATEGORY_USER).from(Self::Table, Self::UserId)
            .to(User::Table, User::Id));
        tcs.primary_key(Index::create().name(PK_PROGRESS_CATEGORY).col(Self::UserId).col(Self::Id));
        tcs
    }
    
    fn index_create_statements_server() -> Vec<IndexCreateStatement> {
        [Self::index_create_statements_default(), vec![
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_CREATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_DELETED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::DeletedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_NAME)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::Name)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_CATEGORY_USER_ID_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::UpdatedAt)
                .to_owned(),            
        ]].concat()
    }
    fn table_drop_statement_server() -> TableDropStatement {
        Self::table_drop_statement_default()
    }


}

#[derive(DeriveIden)]
pub enum ProgressEntry {
    Table,
    #[cfg(feature="server")]
    UserId,
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
#[cfg(feature="server")]
static IDX_PROGRESS_ENTITY_USER_ID_CREATED_AT: &str = "idx_progress_entity_user_id_created_at";
#[cfg(feature="server")]
static IDX_PROGRESS_ENTITY_USER_ID_UPDATED_AT: &str = "idx_progress_entity_user_id_updated_at";
#[cfg(feature="server")]
static IDX_PROGRESS_ENTITY_USER_ID_DELETED_AT: &str = "idx_progress_entity_user_id_deleted_at";
#[cfg(feature="server")]
static IDX_PROGRESS_ENTITY_USER_ID_PROGRESSED_AT: &str = "idx_progress_entity_user_id_progressed_at";
static FK_PROGRESS_ENTITY_PROGRESS_CATEGORY: &str = "fk_progress_entity_progress_category";
#[cfg(feature="server")]
static FK_PROGRESS_ENTITY_USER: &str = "fk_progress_entity_user";
static PK_PROGRESS_ENTITY: &str = "pk_progress_entity";

impl MigrationTableDefault for ProgressEntry {

    fn table_create_statement_default() -> TableCreateStatement {
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

    fn index_create_statements_default() -> Vec<IndexCreateStatement> {
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
    
    fn table_drop_statement_default() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}
#[cfg(feature="client")]
impl MigrationTableClient for ProgressEntry {

    fn table_create_statement_client() -> TableCreateStatement{
        let mut tcs: TableCreateStatement = Self::table_create_statement_default();
        tcs.primary_key(Index::create().name(PK_PROGRESS_ENTITY).col(Self::Id));
        tcs
    }
    fn index_create_statements_client() -> Vec<IndexCreateStatement> {
        Self::index_create_statements_default()
    }
    fn table_drop_statement_client() -> TableDropStatement {
        Self::table_drop_statement_default()
    }
}

#[cfg(feature = "server")]
impl MigrationTableServer for ProgressEntry {

    fn table_create_statement_server() -> TableCreateStatement{
        let mut tcs: TableCreateStatement = Self::table_create_statement_default();
        tcs.col(integer(Self::UserId));
        tcs.foreign_key(ForeignKey::create()
            .name(FK_PROGRESS_ENTITY_USER)
            .from(Self::Table, Self::UserId)
            .to(User::Table, User::Id)
        );
        tcs.primary_key(Index::create().name(PK_PROGRESS_ENTITY).col(Self::UserId).col(Self::Id));

        tcs

    }

    fn index_create_statements_server() -> Vec<IndexCreateStatement> {
        let mut default = Self::index_create_statements_default();
        default.append(&mut vec![
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_CREATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::CreatedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_DELETED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::DeletedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_PROGRESSED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::ProgressedAt)
                .to_owned(),
            Index::create().name(IDX_PROGRESS_ENTITY_USER_ID_UPDATED_AT)
                .table(Self::Table)
                .col(Self::UserId)
                .col(Self::UpdatedAt) 
                .to_owned(),           
        ]);
        default
    }
    
    fn table_drop_statement_server() -> TableDropStatement {
        Self::table_drop_statement_default()
    }


}
