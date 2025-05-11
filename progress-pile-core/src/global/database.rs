use crate::error::Error;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use sea_orm_migration::MigratorTrait;
use tokio::sync::OnceCell;

/* 
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
    
    pub async fn get_or_try_init<T, U>(&self, options: T, _: U) -> Result<&DatabaseConnection, Error> where
    T: Into<ConnectOptions>,
    U: MigratorTrait
    {
    self.inner.get_or_try_init(|| async {
        let db = Database::connect(options).await?;
        U::up(&db, None).await?;
        Ok(db)
    }).await
}

}


pub static DATABASE_CONNECTION: OnceDatabaseConnection = OnceDatabaseConnection::new();
*/

pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub struct GlobalDatabase;

impl GlobalDatabase {
    pub fn get() -> Option<&'static DatabaseConnection> {
        DATABASE_CONNECTION.get()
    }
    
    pub async fn get_or_try_init<T, U>(options: T, _: U) -> Result<&'static DatabaseConnection, Error> where
    T: Into<ConnectOptions>,
    U: MigratorTrait
    {
        DATABASE_CONNECTION.get_or_try_init(|| async {
            let db = Database::connect(options).await?;
            U::up(&db, None).await?;
            Ok(db)
        }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}