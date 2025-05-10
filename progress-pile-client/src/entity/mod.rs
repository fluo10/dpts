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
    use super::*;

    use chrono::Local;
    use sea_orm::entity::*;
    use progress_pile_migration::{ClientMigrator, MigratorTrait};
    use uuid::Uuid;
    use crate::database_connection::DATABASE_CONNECTION;


    #[tokio::test]
    async fn check_insert_entity() {
        let db = DATABASE_CONNECTION.get_or_init("sqlite::memory:").await;
        ClientMigrator::up(db, None).await.unwrap();
        let local_date_time = Local::now();
        let offset_date_time = local_date_time.with_timezone(local_date_time.offset());

        let category = ProgressCategoryActiveModel{
            name: Set("test_category".to_owned()),
            ..ProgressCategoryActiveModel::new()
        }.insert(db).await.unwrap();

        let entry1= ProgressEntryActiveModel {
            progress_category_id: Set(category.id),
            ..ProgressEntryActiveModel::new()
        }.insert(db).await.unwrap();

        ClientMigrator::reset(db).await.unwrap();
        //db.clone().close().await.unwrap();
    }
}