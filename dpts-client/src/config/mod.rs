mod storage;

pub use storage::*;

use crate::error::Error;
use serde::{
    Deserialize,
    Serialize
};

use tokio::sync::OnceCell;

pub static CLIENT_CONFIG: OnceCell<ClientConfig> = OnceCell::const_new();



pub struct ClientConfig {
    pub storage: ClientStorageConfig,
}

impl TryFrom<&PartialClientConfig> for ClientConfig {
    type Error = Error;
    fn try_from(p: &PartialClientConfig) -> Result<ClientConfig, Self::Error> {
        Ok(ClientConfig{
            storage: p.clone().storage.ok_or(Error::MissingConfig("storage".to_string()))?,
        })

    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialClientConfig {
    pub storage: Option<ClientStorageConfig>,
}

impl PartialClientConfig {

}


impl Default for PartialClientConfig {
    fn default() -> Self {
        PartialClientConfig {
            storage: None
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::OnceCell;
    const EMPTY_CONFIG_TOML: &str = r#""#;
    static EMPTY_CONFIG_STRUCT: OnceCell<PartialClientConfig> = OnceCell::const_new();
    
    async fn get_empty_config_struct() -> &'static PartialClientConfig {
        EMPTY_CONFIG_STRUCT.get_or_init(|| async {
            PartialClientConfig{
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
storage = "local"
"#;
    static LOCAL_STORAGE_CONFIG_STRUCT: OnceCell<PartialClientConfig> = OnceCell::const_new();
    
    async fn get_local_storage_client_config_struct() -> &'static PartialClientConfig {
        LOCAL_STORAGE_CONFIG_STRUCT.get_or_init(|| async {
            PartialClientConfig{
                storage: Some(ClientStorageConfig::Local),  
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

    const REMOTE_STORAGE_CONFIG_TOML: &str = r#"time_zone = "UTC"

[storage.remote]
endpoint = "https://example.com"
access_key = "test"
"#;
    static REMOTE_STORAGE_CONFIG_STRUCT: OnceCell<PartialClientConfig> = OnceCell::const_new();
    
    async fn get_remote_storage_client_config_struct() -> &'static PartialClientConfig {
        REMOTE_STORAGE_CONFIG_STRUCT.get_or_init(|| async {
            PartialClientConfig{
                storage: Some(ClientStorageConfig::Remote(ClientRemoteStorageConfig {
                    endpoint: "https://example.com".to_string(),
                    access_key: "test".to_string(),
                })),  
            }
        }).await
    }
    #[tokio::test]
    async fn deserialize_remote_storage_client_config() {
        let config: PartialClientConfig = toml::from_str(REMOTE_STORAGE_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_remote_storage_client_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_remote_storage_client_config() {
        assert_eq!(REMOTE_STORAGE_CONFIG_TOML, toml::to_string(get_remote_storage_client_config_struct().await).unwrap());
    }
}

