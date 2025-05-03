mod storage;

pub use storage::*;

use serde::{
    Deserialize,
    Serialize
};

use chrono_tz::{Tz, UTC};
use crate::{get_host_time_zone_or_utc, Error};
use std::{
    str::FromStr,
    net::IpAddr
};



pub struct ClientConfig {
    
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialClientConfig {
    time_zone: Option<Tz>,
    storage: Option<PartialClientStorageConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::OnceCell;
    use std::path::PathBuf;
    const EMPTY_CONFIG_TOML: &str = r#""#;
    static EMPTY_CONFIG_STRUCT: OnceCell<PartialClientConfig> = OnceCell::const_new();
    
    async fn get_empty_config_struct() -> &'static PartialClientConfig {
        EMPTY_CONFIG_STRUCT.get_or_init(|| async {
            PartialClientConfig{
                time_zone: None,
                storage: None,
            }
        }).await

    }

    #[tokio::test]
    async fn deserialize_empty_client_config() {
        let config: PartialClientConfig = toml::from_str(EMPTY_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_empty_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_empty_client_config() {
        assert_eq!(EMPTY_CONFIG_TOML, toml::to_string(get_empty_config_struct().await).unwrap());
    }

    const LOCAL_STORAGE_CONFIG_TOML: &str = r#"time_zone = "UTC"

[storage.local]
database_path = "."
"#;
    static LOCAL_STORAGE_CONFIG_STRUCT: OnceCell<PartialClientConfig> = OnceCell::const_new();
    
    async fn get_local_storage_client_config_struct() -> &'static PartialClientConfig {
        LOCAL_STORAGE_CONFIG_STRUCT.get_or_init(|| async {
            PartialClientConfig{
                time_zone: Some(UTC),
                storage: Some(PartialClientStorageConfig::Local(
                    PartialClientLocalStorageConfig {
                        database_path: Some(PathBuf::from_str(".").unwrap()),
                    }
                )),  
            }
        }).await
    }
    #[tokio::test]
    async fn deserialize_local_storage_client_config() {
        let config: PartialClientConfig = toml::from_str(LOCAL_STORAGE_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_local_storage_client_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_local_storage_client_config() {
        assert_eq!(LOCAL_STORAGE_CONFIG_TOML, toml::to_string(get_local_storage_client_config_struct().await).unwrap());
    }
}

