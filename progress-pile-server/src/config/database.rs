use std::time::Duration;

use clap::Args;
use sea_orm::ConnectOptions;
use serde::Deserialize;

use crate::error::Error;

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

impl Into<ConnectOptions> for &DatabaseConfig {
    fn into(self) -> ConnectOptions {
        let mut opt = ConnectOptions::new(&self.url);
        if let Some(x) = self.max_connections {
            opt.max_connections(x);
        }
        if let Some(x) = self.min_connections {
            opt.min_connections(x);
        }
        if let Some(x) = self.connect_timeout {
            opt.connect_timeout(x);
        }
        if let Some(x) = self.acquire_timeout {
            opt.acquire_timeout(x);
        }
        if let Some(x) = self.idle_timeout {
            opt.idle_timeout(x);
        }
        if let Some(x) = self.max_lifetime {
            opt.max_lifetime(x);
        }
        opt.sqlx_logging(self.sqlx_logging);
        
        opt
    }
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
#[derive(Args)]
pub struct PartialDatabaseConfig {
    #[arg(long)]
    pub url: Option<String>,
    #[arg(long)]
    pub max_connections: Option<u32>,
    #[arg(long)]
    pub min_connections: Option<u32>,
    #[arg(long, value_parser = parse_duration )]
    pub connect_timeout: Option<Duration>,
    #[arg(long, value_parser = parse_duration )]
    pub acquire_timeout: Option<Duration>,
    #[arg(long, value_parser = parse_duration )]
    pub idle_timeout: Option<Duration>,
    #[arg(long, value_parser = parse_duration )]
    pub max_lifetime: Option<Duration>,
    #[arg(long)]
    pub sqlx_logging: Option<bool>
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, Error> {
    let seconds = arg.parse()?;
    Ok(std::time::Duration::from_secs(seconds))
}

