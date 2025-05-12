use crate::error::Error;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use sea_orm_migration::MigratorTrait;
use tokio::sync::OnceCell;


pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub trait GlobalDatabase {
    fn get(&self) -> Option<&DatabaseConnection>;
    async fn get_or_try_init(&self) -> Result<&DatabaseConnection, Error>;
    async fn get_or_try_init_with_connect_options<T>(&self, options: T) -> Result<&DatabaseConnection, Error> where 
        T: Into<ConnectOptions>;
}

#[cfg(test)]
mod tests {
    use super::*;
}