mod config;
#[cfg(feature="desktop")]
mod database;

pub use config::GlobalConfig;

#[cfg(feature="desktop")]
pub use database::GlobalDatabase;
