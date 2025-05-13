#[cfg(any(feature="sqlite", feature="postgres"))]
mod database;

#[cfg(any(feature="sqlite", feature="postgres"))]
pub use database::{
    DatabaseConfig,
    PartialDatabaseConfig,
};
