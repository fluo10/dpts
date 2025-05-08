use sea_orm_migration::{prelude::*, schema::*};


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
impl User {
    pub fn server_table_create_statement() -> TableCreateStatement{
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

    pub fn table_drop_statement() -> TableDropStatement {
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
    AccessToken,
    Note,
}

#[cfg(feature="server")]
impl AccessToken {
    pub fn server_table_create_statement() -> TableCreateStatement {
        Table::create()
                    .table(Self::Table)
                    .if_not_exists()
                    .col(pk_auto(Self::Id))
                    .col(timestamp_with_time_zone(Self::CreatedAt))
                    .col(timestamp_with_time_zone(Self::UpdatedAt))
                    .col(timestamp_with_time_zone_null(Self::ExpiredAt))
                    .col(string(Self::AccessToken))
                    .col(string(Self::Note))
                    .to_owned()
    }

    pub fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[derive(DeriveIden)]
pub enum ProgressCategory {
    Table,
    #[cfg(feature="server")]
    Id,
    #[cfg(feature="server")]
    UserId,
    Uuid,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    Name,
}

impl ProgressCategory {
    fn default_table_create_statement() -> TableCreateStatement {
        Table::create()
                    .table(Self::Table)
                    .if_not_exists()
                    .col(timestamp_with_time_zone(Self::CreatedAt))
                    .col(timestamp_with_time_zone(Self::UpdatedAt))
                    .col(timestamp_with_time_zone_null(Self::DeletedAt))
                    .to_owned()
    }
    #[cfg(feature="client")]
    pub fn client_table_create_statement() -> TableCreateStatement{
        let mut tcs = Self::default_table_create_statement();
        tcs.col(pk_uuid(Self::Uuid))
            .col(string_uniq(Self::Name));
        tcs
    }

    #[cfg(feature="server")]
    pub fn server_table_create_statement() -> TableCreateStatement{
        todo!()
    }

    pub fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}

#[derive(DeriveIden)]
pub enum ProgressEntry {
    Table,
    #[cfg(feature="server")]
    Id,
    #[cfg(feature="server")]
    UserId,
    Uuid,
    ProgressCategoryUuid,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    EnteredAt,
    Quantity,
    Note,
}

static PROGRESS_ENTRY_PROGRESS_CATEGORY_FOREIGN_KEY_NAME: &str = "fk__progress_entry__progress__category";


impl ProgressEntry {
    fn default_table_create_statement() -> TableCreateStatement {
        Table::create()
        .table(Self::Table)
        .if_not_exists()
        .col(uuid(Self::ProgressCategoryUuid))
        .col(timestamp_with_time_zone(Self::CreatedAt))
        .col(timestamp_with_time_zone(Self::UpdatedAt))
        .col(timestamp_with_time_zone_null(Self::DeletedAt))
        .col(timestamp_with_time_zone(Self::EnteredAt))
        .col(integer(Self::Quantity))
        .col(string(Self::Note))

        .to_owned()
    }
    #[cfg(feature="client")]
    pub fn client_table_create_statement() -> TableCreateStatement{
        let mut tcs = Self::default_table_create_statement();
        tcs.col(pk_uuid(Self::Uuid))
            .foreign_key(ForeignKey::create()
                .name(PROGRESS_ENTRY_PROGRESS_CATEGORY_FOREIGN_KEY_NAME)
                .from(Self::Table, Self::ProgressCategoryUuid)
                .to(ProgressCategory::Table, ProgressCategory::Uuid)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
            );
        tcs
    }

    #[cfg(feature="server")]
    pub fn server_table_create_statement() -> TableCreateStatement{
        todo!()
    }
    pub fn table_drop_statement() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}
