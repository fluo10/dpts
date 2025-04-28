use core::time::Duration;
use sea_orm::{ConnectOptions, Database};
use dpts_migration::{Migrator, MigratorTrait};

#[tokio::test]
async fn main() {
    let mut opt = ConnectOptions::new("sqlite::memory:");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
        //.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    let db= Database::connect(opt).await.unwrap();
    Migrator::fresh(&db).await.unwrap();
    Migrator::reset(&db).await.unwrap();
    db.close().await.unwrap();
}