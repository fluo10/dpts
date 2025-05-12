mod progress_category;
mod progress_entry;

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    use chrono::Local;
    use progress_pile_core::global::GlobalDatabase;
    use sea_orm::{entity::*, ConnectOptions, Database, DatabaseConnection};
    use progress_pile_migration::{ClientMigrator, MigratorTrait};
    use uuid::Uuid;
    use crate::error::Error;
    use crate::global::GLOBAL;

     #[tokio::test]
    async fn check_insert_entity() {
        let db = GLOBAL.get_or_init_temporary_database().await;
        
        let category = ProgressCategoryActiveModel{
            name: Set("test_category".to_owned()),
            ..ProgressCategoryActiveModel::new()
        }.insert(db).await.unwrap();

        let entry1= ProgressEntryActiveModel {
            progress_category_id: Set(category.id),
            ..ProgressEntryActiveModel::new()
        }.insert(db).await.unwrap();
    }

}