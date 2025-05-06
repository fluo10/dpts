mod storage;

pub use dpts_core::config::*;
pub use storage::*;



use crate::error::Error;
use serde::{
    Deserialize,
    Serialize
};

use tokio::sync::OnceCell;

pub static CLIENT_CONFIG: OnceCell<ClientConfig> = OnceCell::const_new();

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]

pub struct Config {
    pub client: ClientConfig,
    pub global: PartialGlobalConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClientConfig {
    pub storage: ClientStorageConfig,
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::UTC;
    use tokio::sync::OnceCell;

    const LOCAL_STORAGE_CONFIG_TOML: &str = r#"[client]
storage = "local"

[global]
"#;
    static LOCAL_STORAGE_CONFIG_STRUCT: OnceCell<Config> = OnceCell::const_new();
    
    async fn get_local_storage_client_config_struct() -> &'static Config {
        LOCAL_STORAGE_CONFIG_STRUCT.get_or_init(|| async {
            Config{
                client: ClientConfig{
                    storage: ClientStorageConfig::Local,
                },   
                global: PartialGlobalConfig { time_zone: None },
            }
        }).await
    }
    #[tokio::test]
    async fn deserialize_local_storage_client_config() {
        let config: Config = toml::from_str(LOCAL_STORAGE_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_local_storage_client_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_local_storage_client_config() {
        assert_eq!(LOCAL_STORAGE_CONFIG_TOML, toml::to_string(get_local_storage_client_config_struct().await).unwrap());
    }

    const REMOTE_STORAGE_CONFIG_TOML: &str = r#"[client.storage.remote]
endpoint = "https://example.com"
access_key = "test"

[global]
time_zone = "UTC"
"#;
    static REMOTE_STORAGE_CONFIG_STRUCT: OnceCell<Config> = OnceCell::const_new();
    
    async fn get_remote_storage_client_config_struct() -> &'static Config {
        REMOTE_STORAGE_CONFIG_STRUCT.get_or_init(|| async {
            Config{
                client: ClientConfig {
                    storage: ClientStorageConfig::Remote(ClientRemoteStorageConfig {
                        endpoint: "https://example.com".to_string(),
                        access_key: "test".to_string(),
                    })
                },
                global: PartialGlobalConfig { time_zone: Some(UTC) }  
            }
        }).await
    }
    #[tokio::test]
    async fn deserialize_remote_storage_client_config() {
        let config: Config = toml::from_str(REMOTE_STORAGE_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_remote_storage_client_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_remote_storage_client_config() {
        assert_eq!(REMOTE_STORAGE_CONFIG_TOML, toml::to_string(get_remote_storage_client_config_struct().await).unwrap());
    }
}

