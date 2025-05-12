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
    use progress_pile_migration::{ServerMigrator, MigratorTrait};
    use crate::{entity::*, global::GLOBAL};

    #[tokio::test]
    async fn check_database_connection() {
        
        let db = GLOBAL.get_or_init_temporary_database().await;
        assert!(db.ping().await.is_ok());
    }
    #[tokio::test]
    async fn check_insert_entity() {
        let db = GLOBAL.get_or_init_temporary_database().await;
        
        let timestamp = Local::now().fixed_offset();

        let user = UserActiveModel{
            login_name: Set("admin".to_owned()),
            password_hash: Set("admin".to_owned()),
            created_at: Set(timestamp),
            updated_at: Set(timestamp),
            ..UserActiveModel::new()
        }.insert(db).await.unwrap();


        let access_token = AccessTokenActiveModel{
            user_id: Set(user.id),
            ..Default::default()
        }.insert(db).await.unwrap();

        let progress_category = ProgressCategoryActiveModel{
            user_id: Set(user.id),
            name: Set("test_category".to_string()),
            ..Default::default()
        }.insert(db)
        .await.unwrap();

        ProgressEntryActiveModel {
            user_id: Set(user.id),
            progress_category_id: Set(progress_category.id),
            ..Default::default()
        }.insert(db)
        .await.unwrap();

    }
}