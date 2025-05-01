use dpts_error::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub listen_ips: Vec<String>,
    pub port: u16,
    pub database_url: String
}

#[derive(Deserialize)]

pub struct PartialServerConfig {
    pub listen_ips: Option<Vec<String>>,
    pub port: Option<u16>,
    pub database_url: Option<String>,
}

impl PartialServerConfig {
    /// #Examples
    /// ```
    /// use dpts_config::PartialServerConfig;
    /// let config = PartialServerConfig::try_from_toml(r#"
    /// listen_ips = ["0.0.0.0"]
    /// port = 8000
    /// database_url = "sqlite::memory:"
    /// "#).unwrap();
    /// assert_eq!(config.listen_ips, Some(vec!["0.0.0.0".to_string()]));
    /// assert_eq!(config.port, Some(8000));
    /// assert_eq!(config.database_url,  Some("sqlite::memory:".to_string()));
    /// ```
    /// 
    pub fn try_from_toml(s: &str) -> Result<Self, Error> {
        Ok(toml::from_str(s)?)
    }
}

