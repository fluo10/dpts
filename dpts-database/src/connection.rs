use std::time::Duration;
use sea_orm::{entity::*, query::*, ConnectOptions, Database, DatabaseConnection};
use dpts_migration::{Migrator, MigratorTrait};
use dpts_config::ServerConfig;

use tokio::sync::OnceCell;

pub struct OnceDatabaseConnection {
    inner: OnceCell<DatabaseConnection>,
}

impl OnceDatabaseConnection {
    const fn new() -> Self {
        Self {
            inner: OnceCell::const_new(),
        }
    }
    pub fn get(&self) -> Option<&DatabaseConnection> {
        self.inner.get()
    }
    pub async fn get_or_init<F, T>(&self, f: F) -> &DatabaseConnection where 
    F: FnOnce() -> T,
    T: Future<Output = DatabaseConnection> 
    {
        self.inner.get_or_init(f).await
    }

    pub async fn get_or_init_with_server_config(&self, c: &ServerConfig) -> &DatabaseConnection {
        self.get_or_init( || async {
            let mut opt = ConnectOptions::new(&c.database_url);
            opt.max_connections(100)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(8))
                .acquire_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .max_lifetime(Duration::from_secs(8))
                .sqlx_logging(true)
                .sqlx_logging_level(log::LevelFilter::Info);
                //.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
            let db = Database::connect(opt).await.unwrap();
            Migrator::fresh(&db).await.unwrap();
            db
        }).await
    }
    pub async fn get_or_init_with_static_server_config(&self) -> &DatabaseConnection {
        self.get_or_init_with_server_config(dpts_config::SERVER_CONFIG.get().unwrap()).await
    }

    #[cfg(test)]
    pub async fn init_test(&self)  {
        self.get_or_init( || async {
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
            let db = Database::connect(opt).await.unwrap();
            Migrator::fresh(&db).await.unwrap();
            db
        }).await;
            
    }

}


pub static DATABASE_CONNECTION: OnceDatabaseConnection = OnceDatabaseConnection::new();

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;
    use chrono::{offset, FixedOffset, Local, TimeZone};
    use sea_orm::{entity::*, query::*, ConnectOptions, Database};
    use dpts_migration::{Migrator, MigratorTrait};
    use dpts_entity::*;

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