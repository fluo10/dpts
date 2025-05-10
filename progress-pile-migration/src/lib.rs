pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

#[cfg(feature="client")]
pub struct ClientMigrator;

#[cfg(feature="client")]
#[async_trait::async_trait]
impl MigratorTrait for ClientMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::ClientMigration),
        ]
    }
}

#[cfg(feature="server")]
pub struct ServerMigrator;

#[cfg(feature="server")]
#[async_trait::async_trait]
impl MigratorTrait for ServerMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::ServerMigration),
        ]
    }
}


#[cfg(test)]
mod tests {
    use sea_orm_migration::sea_orm::{ConnectOptions, Database};

    use super::*;
    #[cfg(feature="client")]
    #[async_std::test]
    async fn client_migration() {
        let db = Database::connect(ConnectOptions::new("sqlite::memory:")).await.unwrap();
        ClientMigrator::up(&db, None).await.unwrap();
        ClientMigrator::down(&db, None).await.unwrap();
    }
    #[cfg(feature="server")]

    #[async_std::test]
    async fn server_migration() {
        let db = Database::connect(ConnectOptions::new("sqlite::memory:")).await.unwrap();
        ServerMigrator::up(&db, None).await.unwrap();
        ServerMigrator::down(&db, None).await.unwrap();
    }
}