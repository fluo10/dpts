use std::time::Duration;

use serde::Deserialize;

use crate::Error;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<Duration>,
    pub acquire_timeout: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
    pub sqlx_logging: bool,
}

impl TryFrom<PartialDatabaseConfig> for DatabaseConfig{
    type Error = Error;
    fn try_from(p: PartialDatabaseConfig) -> Result<DatabaseConfig, Self::Error> {
        Ok(DatabaseConfig{
            url: p.url.ok_or(Error::MissingConfig("url".to_string()))?,
            max_connections: p.max_connections,
            min_connections: p.min_connections,
            connect_timeout: p.connect_timeout,
            acquire_timeout: p.acquire_timeout,
            idle_timeout: p.idle_timeout,
            max_lifetime: p.max_lifetime,
            sqlx_logging: p.sqlx_logging.ok_or(Error::MissingConfig("sqlx_logging".to_string()))?
        })
    }
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PartialDatabaseConfig {
    pub url: Option<String>,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout: Option<Duration>,
    pub acquire_timeout: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
    pub sqlx_logging: Option<bool>
}

