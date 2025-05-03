use chrono_tz::{Tz, UTC};
use crate::{
    get_host_time_zone_or_utc,
    DatabaseConfig,
    PartialDatabaseConfig,
    Error
};
use serde::Deserialize;
use std::{
    default::Default,
    str::FromStr,
    net::IpAddr,
};
use tokio::sync::OnceCell;

pub static SERVER_CONFIG: OnceServerConfig = OnceServerConfig::const_new();

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ServerConfig {
    pub listen_ips: Vec<IpAddr>,
    pub port: u16,
    pub time_zone: Tz,
    pub database: DatabaseConfig
}

impl ServerConfig {

}

impl TryFrom<PartialServerConfig> for ServerConfig {
    type Error = Error;
    fn try_from(p: PartialServerConfig) -> Result<ServerConfig, Error>{
        Ok(ServerConfig{
            listen_ips: p.listen_ips.ok_or(Error::MissingConfig("listen_ips".to_string()))?,
            port: p.port.ok_or(Error::MissingConfig("port".to_string()))?,
            database: p.database.ok_or(Error::MissingConfig("database.*".to_string()))?.try_into()?,
            time_zone: p.time_zone.ok_or(Error::MissingConfig("time_zone".to_string()))?,
        })
    }
}

pub struct OnceServerConfig {
    inner: OnceCell<ServerConfig>,
}

impl OnceServerConfig {
    const fn const_new() -> Self {
        Self {
            inner: OnceCell::const_new(),
        }
    }
    pub fn get(&self) -> Option<&ServerConfig> {
        self.inner.get()
    }
    pub async fn get_or_init<F, T>(&self, f: F) -> &ServerConfig where 
    F: FnOnce() -> T,
    T: Future<Output = ServerConfig> 
    {
        self.inner.get_or_init(f).await
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]

pub struct PartialServerConfig {
    pub listen_ips: Option<Vec<IpAddr>>,
    pub port: Option<u16>,
    pub database: Option<PartialDatabaseConfig>,
    pub time_zone: Option<Tz>,
}

impl PartialServerConfig {
   
    pub fn try_from_toml(s: &str) -> Result<Self, Error> {
        Ok(toml::from_str(s)?)
    }
}

impl Default for PartialServerConfig {
    fn default() -> Self {
        PartialServerConfig {
            listen_ips: Some(vec!["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()]),
            port: Some(3000),
            database: None,
            time_zone: Some(get_host_time_zone_or_utc())
        }
    }
}

impl FromStr for PartialServerConfig {
    type Err = Error;
    /// #Examples
    /// ```
    /// use dpts_config::{
    ///     PartialServerConfig,
    ///     PartialDatabaseConfig,
    /// };
    /// use std::{
    ///     default::Default,
    ///     net::IpAddr
    /// };
    /// let config_from_str: PartialServerConfig = r#"
    /// listen_ips = ["0.0.0.0"]
    /// port = 8000
    /// time_zone = "Asia/Tokyo"
    /// 
    /// [database]
    /// url = "sqlite::memory:"
    /// sqlx_logging = true
    /// "#.parse().unwrap();
    /// 
    /// let config: PartialServerConfig = PartialServerConfig{
    ///     listen_ips : Some(vec!["0.0.0.0".parse().unwrap()]),
    ///     port: Some(8000),
    ///     time_zone: Some(chrono_tz::Asia::Tokyo),
    ///     database: Some(PartialDatabaseConfig {
    ///         url: Some("sqlite::memory:".to_string()),
    ///         sqlx_logging: Some(true),
    ///         ..Default::default()
    ///     }),
    /// };
    /// 
    /// assert_eq!(config_from_str, config);
    /// ```
    /// 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}
