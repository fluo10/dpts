mod config;

#[cfg(any(feature="sqlite", feature="postgres"))]
mod database;

pub use config::GlobalConfig;

#[cfg(any(feature="sqlite", feature="postgres"))]
pub use database::GlobalDatabase;
