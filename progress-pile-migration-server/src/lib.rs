pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
        ]
    }
}


#[cfg(test)]
mod tests {
    use sea_orm_migration::sea_orm::{ConnectOptions, Database};

    use super::*;
    #[async_std::test]
    async fn test_migration_server() {
        let db = Database::connect(ConnectOptions::new("sqlite::memory:")).await.unwrap();
        Migrator::up(&db, None).await.unwrap();
        Migrator::down(&db, None).await.unwrap();
    }
}