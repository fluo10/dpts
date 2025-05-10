use crate::config::{
    DATABASE_CONFIG,
    DatabaseConfig
};
use std::time::Duration;
use sea_orm::{entity::*, query::*, ConnectOptions, Database, DatabaseConnection};

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

    pub async fn get_or_init<T>(&self, c: T) -> &DatabaseConnection where
    T: Into<ConnectOptions> 
    {
        self.inner.get_or_init(|| async {
            Database::connect(c).await.unwrap()
        }).await
    }
}


pub static DATABASE_CONNECTION: OnceDatabaseConnection = OnceDatabaseConnection::new();

#[cfg(test)]
mod tests {
    use super::*;

    //#[tokio::test]
    async fn check_database_connection() {
        DATABASE_CONNECTION.get_or_init("sqlite::memory:").await;
        let db = DATABASE_CONNECTION.get().unwrap();
        assert!(db.ping().await.is_ok());
    }
}