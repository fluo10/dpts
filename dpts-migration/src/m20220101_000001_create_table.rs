use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(pk_auto(User::Id))
                .col(string_uniq(User::LoginName))
                .col(string(User::PasswordHash))
                .col(timestamp_with_time_zone(User::CreatedAt))
                .col(timestamp_with_time_zone(User::UpdatedAt))
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(RecordHeader::Table)
                .if_not_exists()
                .col(pk_auto(RecordHeader::Id))
                .col(integer(RecordHeader::UserId))
                .col(timestamp_with_time_zone(RecordHeader::RecordedAt))
                .col(timestamp_with_time_zone(RecordHeader::CreatedAt))
                .col(timestamp_with_time_zone(RecordHeader::UpdatedAt))
                .col(string(RecordHeader::Comment))
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_RecordHeader_User")
                        .from(RecordHeader::Table, RecordHeader::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(RecordTag::Table)
                .if_not_exists()
                .col(pk_auto(RecordTag::Id))
                .col(integer(RecordTag::UserId))
                .col(string(RecordTag::Name))
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_RecordTag_User")
                        .from(RecordTag::Table, RecordHeader::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(RecordDetail::Table)
                .if_not_exists()
                .col(pk_auto(RecordDetail::Id))
                .col(integer(RecordDetail::RecordHeaderId))
                .col(integer(RecordDetail::RecordTagId))
                .col(integer(RecordDetail::Count))
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_RecordDetail_RecordHeader")
                        .from(RecordDetail::Table, RecordDetail::RecordHeaderId)
                        .to(RecordHeader::Table, RecordHeader::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_RecordDetail_RecordTag")
                        .from(RecordDetail::Table, RecordDetail::RecordTagId)
                        .to(RecordTag::Table, RecordTag::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                )
                .to_owned(),
        ).await?;



        manager.create_index(
            Index::create()
                .name("IDX_User_LoginName")
                .table(User::Table)
                .col(User::LoginName)
                .to_owned(),
        ).await?;

        manager.create_index(
            Index::create()
                .name("IDX_RecordHeader_RecordedAt")
                .table(RecordHeader::Table)
                .col(RecordHeader::RecordedAt)
                .to_owned(),
        ).await?;
        
        manager.create_index(
            Index::create()
                .name("IDX_RecordTag_Name")
                .table(RecordTag::Table)
                .col(RecordTag::Name)
                .to_owned(),
        ).await?;

        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.drop_index(Index::drop().name("IDX_User_LoginName").to_owned()).await?;
        manager.drop_index(Index::drop().name("IDX_RecordHeader_RecordedAt").to_owned()).await?;
        manager.drop_index(Index::drop().name("IDX_RecordTag_Name").to_owned()).await?;


        manager.drop_table(
            Table::drop().table(RecordDetail::Table).to_owned()
        ).await?;
        manager.drop_table(
            Table::drop().table(RecordTag::Table).to_owned()
        ).await?;
        manager.drop_table(
            Table::drop().table(RecordHeader::Table).to_owned()
        ).await?;
        manager.drop_table(
            Table::drop().table(User::Table).to_owned()
        ).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    LoginName,
    PasswordHash,
}

#[derive(DeriveIden)]
enum RecordHeader {
    Table,
    Id,
    UserId,
    CreatedAt,
    UpdatedAt,
    RecordedAt,
    Comment,
}

#[derive(DeriveIden)]
enum RecordTag {
    Table,
    Id,
    UserId,
    Name,
}

#[derive(DeriveIden)]
enum RecordDetail {
    Table,
    Id,
    RecordHeaderId,
    RecordTagId,
    Count,
}

