use chrono_tz::{Tz, UTC};
use crate::{get_host_time_zone_or_utc, Error};
use serde::Deserialize;
use std::{
    str::FromStr,
    net::IpAddr
};
use tokio::sync::OnceCell;

pub static SERVER_CONFIG: OnceServerConfig = OnceServerConfig::const_new();

#[derive(Deserialize)]
pub struct ServerConfig {
    pub listen_ips: Vec<IpAddr>,
    pub port: u16,
    pub database_url: String,
    pub time_zone: Tz,
}

impl ServerConfig {

}

impl TryFrom<PartialServerConfig> for ServerConfig {
    type Error = Error;
    fn try_from(p: PartialServerConfig) -> Result<ServerConfig, Error>{
        Ok(ServerConfig{
            listen_ips: p.listen_ips.ok_or(Error::MissingConfig("listen_ips".to_string()))?,
            port: p.port.ok_or(Error::MissingConfig("port".to_string()))?,
            database_url: p.database_url.ok_or(Error::MissingConfig("database_url".to_string()))?,
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

#[derive(Deserialize)]

pub struct PartialServerConfig {
    pub listen_ips: Option<Vec<IpAddr>>,
    pub port: Option<u16>,
    pub database_url: Option<String>,
    pub time_zone: Option<Tz>,
}

impl PartialServerConfig {
   
    pub fn try_from_toml(s: &str) -> Result<Self, Error> {
        Ok(toml::from_str(s)?)
    }
}

impl Default for PartialServerConfig {
    /// #Examples
    /// ```
    /// use dpts_config::PartialServerConfig;
    /// let config = PartialServerConfig::default();
    /// assert_eq!(config.listen_ips, Some(vec!["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()]));
    /// assert_eq!(config.port, Some(3000));
    /// assert_eq!(config.database_url,  None);
    /// assert_eq!(config.time_zone, Some(iana_time_zone::get_timezone().unwrap().parse().unwrap()))
    /// ```
    /// 
    fn default() -> Self {
        PartialServerConfig {
            listen_ips: Some(vec!["127.0.0.1".parse().unwrap(), "::1".parse().unwrap()]),
            port: Some(3000),
            database_url: None,
            time_zone: Some(get_host_time_zone_or_utc())
        }
    }
}

impl FromStr for PartialServerConfig {
    type Err = Error;
    /// #Examples
    /// ```
    /// use dpts_config::PartialServerConfig;
    /// let config: PartialServerConfig = r#"
    /// listen_ips = ["0.0.0.0"]
    /// port = 8000
    /// database_url = "sqlite::memory:"
    /// time_zone = "Asia/Tokyo"
    /// "#.parse().unwrap();
    /// assert_eq!(config.listen_ips, Some(vec!["0.0.0.0".parse().unwrap()]));
    /// assert_eq!(config.port, Some(8000));
    /// assert_eq!(config.database_url,  Some("sqlite::memory:".to_string()));
    /// assert_eq!(config.time_zone, Some("Asia/Tokyo".parse().unwrap()))
    /// ```
    /// 
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(toml::from_str(s)?)
    }
}
