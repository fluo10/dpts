use progress_pile_migration::{ClientMigrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::error::Error;
use tokio::sync::OnceCell;
use progress_pile_core::global::GlobalDatabase;


pub static GLOBAL_DATABASE: OnceGlobalDatabase = OnceGlobalDatabase{
    inner: OnceCell::const_new(),
};
pub struct OnceGlobalDatabase {
    inner: OnceCell<DatabaseConnection>,
}

impl OnceGlobalDatabase {

}


impl GlobalDatabase for OnceGlobalDatabase {
    fn get(&self) -> Option<& DatabaseConnection> {
        self.inner.get()
    }
    async fn get_or_try_init(&self) -> Result<&DatabaseConnection, Error> {
        todo!()
    }
    
    async fn get_or_try_init_with_connect_options<T>(&self, options: T) -> Result<&DatabaseConnection, Error> where
    T: Into<ConnectOptions>,
    {
        Ok(self.inner.get_or_try_init(|| async {
            let db = Database::connect(options).await?;
            ClientMigrator::up(&db, None).await?;
            Ok::<DatabaseConnection, Error>(db)
        }).await?)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::LazyLock;

    use tokio::sync::OnceCell;
    use super::*;

    pub static TEST_DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
        let mut temp_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();
        temp_path.disable_cleanup(true);
        let url = "sqlite://".to_string() + temp_path.as_os_str().to_str().unwrap() + "?mode=rwc";
        println!("{}", &url);
        url
    });
    
    impl OnceGlobalDatabase {
        pub async fn get_or_init_temp(&self) -> &DatabaseConnection {

            self.get_or_try_init_with_connect_options(&*TEST_DATABASE_URL).await.unwrap()
        }
    }

    #[tokio::test]
    async fn connect_database () {
        let db = GLOBAL_DATABASE.get_or_init_temp().await;
        assert!(db.ping().await.is_ok());
    }

}