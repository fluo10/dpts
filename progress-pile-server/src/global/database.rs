use progress_pile_migration::{ServerMigrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::error::Error;
use tokio::sync::OnceCell;
use progress_pile_core::global::GlobalDatabase;

use super::Global;

impl GlobalDatabase for Global {
    fn get_database(&self) -> Option<&DatabaseConnection> {
        self.database.get()
    }
    async fn get_or_try_init_database(&self) -> Result<&DatabaseConnection, Error> {
        todo!()
    }
    
    async fn get_or_try_init_database_with_connect_options<T>(&self, options: T) -> Result<&DatabaseConnection, Error> where
    T: Into<ConnectOptions>,
    {
        Ok(self.database.get_or_try_init(|| async {
            let db = Database::connect(options).await?;
            ServerMigrator::up(&db, None).await?;
            Ok::<DatabaseConnection, Error>(db)
        }).await?)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::LazyLock;

    use crate::global::GLOBAL;

    use super::*;

    pub static TEST_DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
        let mut temp_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();
        temp_path.disable_cleanup(true);
        let url = "sqlite://".to_string() + temp_path.as_os_str().to_str().unwrap() + "?mode=rwc";
        println!("{}", &url);
        url
    });
    
    impl Global {
        pub async fn get_or_init_temporary_database(&self) -> &DatabaseConnection {

            self.get_or_try_init_database_with_connect_options(&*TEST_DATABASE_URL).await.unwrap()
        }
    }

    #[tokio::test]
    async fn connect_database () {
        let db = GLOBAL.get_or_init_temporary_database().await;
        assert!(db.ping().await.is_ok());
    }

}