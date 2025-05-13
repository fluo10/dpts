use crate::config::ClientConfig;
#[cfg(feature="desktop")]
use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

#[cfg(feature="desktop")]
mod database;

pub static GLOBAL: Global = Global{
    config: OnceCell::const_new(),
    #[cfg(feature="desktop")]
    database: OnceCell::const_new(),
};
pub struct Global {
    config: OnceCell<ClientConfig>,
    #[cfg(feature="desktop")]
    database: OnceCell<DatabaseConnection>,
}
