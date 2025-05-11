mod access_token;
mod progress_category;
mod progress_entry;
mod user;


pub use access_token::{
    ActiveModel as AccessTokenActiveModel,
    Column as AccessTokenColumn,
    Entity as AccessTokenEntity,
    Model as AccessTokenModel,
};

pub use progress_category::{
    ActiveModel as ProgressCategoryActiveModel,
    Column as ProgressCategoryColumn,
    Entity as ProgressCategoryEntity,
    Model as ProgressCategoryModel,
};

pub use progress_entry::{
    ActiveModel as ProgressEntryActiveModel,
    Column as ProgressEntryColumn,
    Entity as ProgressEntryEntity,
    Model as ProgressEntryModel,
};


pub use user::{
    ActiveModel as UserActiveModel,
    Column as UserColumn,
    Entity as UserEntity,
    Model as UserModel,
};


pub use progress_pile_core::entity::*;



#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;
    use chrono::{offset, FixedOffset, Local, TimeZone};
    use sea_orm::{entity::*, query::*, ConnectOptions, Database};
    use progress_pile_migration_server::{Migrator, MigratorTrait};
    use crate::entity::*;

    #[tokio::test]
    async fn check_database_connection() {
        DATABASE_CONNECTION.init_test().await;
        let db = DATABASE_CONNECTION.get().unwrap();
        assert!(db.ping().await.is_ok());
    }
    #[tokio::test]
    async fn check_insert_entity() {
        DATABASE_CONNECTION.init_test().await;
        let db = DATABASE_CONNECTION.get().unwrap();
        

        let local_date_time = Local::now();
        let offset_date_time = local_date_time.with_timezone(local_date_time.offset());

        let user = UserActiveModel{
            login_name: Set("admin".to_owned()),
            password_hash: Set("admin".to_owned()),
            created_at: Set(offset_date_time),
            updated_at: Set(offset_date_time),
            ..Default::default()
        }.insert(db)
        .await.unwrap();


        let record_tag = RecordTagActiveModel{
            user_id: Set(user.id),
            name: Set("test".to_owned()),
            ..Default::default()
        }.insert(db)
        .await.unwrap();

        let record_header = RecordHeaderActiveModel{
            user_id: Set(user.id),
            created_at: Set(offset_date_time),
            updated_at: Set(offset_date_time),
            recorded_at: Set(offset_date_time),
            comment: Set("".to_owned()),
            ..Default::default()
        }.insert(db)
        .await.unwrap();

        RecordDetailActiveModel {
            record_header_id: Set(record_header.id),
            record_tag_id: Set(record_tag.id),
            count: Set(1),
            ..Default::default()
        }.insert(db)
        .await.unwrap();
        RecordDetailActiveModel {
            record_header_id: Set(record_header.id),
            record_tag_id: Set(record_tag.id),
            count: Set(2),
            ..Default::default()
        }.insert(db)
        .await.unwrap();


        Migrator::reset(db).await.unwrap();
        db.clone().close().await.unwrap();
    }
}