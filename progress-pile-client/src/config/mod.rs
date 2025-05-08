mod storage;

use std::{fmt::{Display, Formatter}, path::{Path, PathBuf}, str::FromStr};

pub use progress_pile_core::config::*;
pub use storage::*;

use crate::error::Error;
use serde::{
    Deserialize,
    Serialize
};

use tokio::{fs::{self, File}, io::AsyncWriteExt, sync::OnceCell};

pub static DEFAULT_CONFIG_FILE_NAME: &str = "dpts_client.toml";
pub static DEFAULT_CONFIG_DIR_NAME: &str = "dpts";

pub static CLIENT_CONFIG: OnceCell<ClientConfig> = OnceCell::const_new();

pub fn get_default_config_file_path() -> Result<PathBuf, Error> {
    let config_dir = dirs::config_dir().ok_or(Error::DefaultConfigDir)?;
    Ok(config_dir.join(DEFAULT_CONFIG_DIR_NAME).join(DEFAULT_CONFIG_FILE_NAME))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]

pub struct Config {
    pub client: ClientConfig,
    pub global: GlobalConfig,
}

impl Config {
    pub async fn read_from_default_toml() -> Result<Self, Error> {
        Ok(Self::read_from_toml(&get_default_config_file_path()?).await?)
    }
    pub async fn write_to_default_toml(&self) -> Result<(), Error> {
        Ok(self.write_to_toml(&get_default_config_file_path()?).await?)
    }
    pub async fn write_to_toml(&self, p: &Path) -> Result<(), Error> {
        let toml = self.to_toml()?;
        let mut file = File::create(p).await?;
        file.write_all(toml.as_bytes()).await?;
        Ok(())
    }
    pub async fn read_from_toml(p: &Path) -> Result<Self, Error> {
        let raw = fs::read_to_string(p).await?;
        Ok(Self::from_toml(&raw)?)
    }
    pub fn from_toml(s: &str) -> Result<Self, Error> {
        Ok(toml::from_str(s)?)
    }
    pub fn to_toml(&self) -> Result<String, Error> {
        Ok(toml::to_string(self)?)
    }
    pub fn set_global(self) -> Result<(), Error> {
        CLIENT_CONFIG.set(self.client)?;
        GLOBAL_CONFIG.set(self.global)?;
        Ok(())
    }
    pub fn from_global() -> Result<Self, Error> {
        Ok(Self {
            client: CLIENT_CONFIG.get().ok_or(Error::UninitializedOnceCell("CLIENT_CONFIG".to_string()))?.clone(),
            global: GLOBAL_CONFIG.get().ok_or(Error::UninitializedOnceCell("GLOBAL_CONFIG".to_string()))?.clone(),
        })
    }
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
                global: GlobalConfig { time_zone: None },
            }
        }).await
    }
    #[tokio::test]
    async fn deserialize_local_storage_client_config() {
        let config: Config = Config::from_toml(LOCAL_STORAGE_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_local_storage_client_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_local_storage_client_config() {
        assert_eq!(LOCAL_STORAGE_CONFIG_TOML, get_local_storage_client_config_struct().await.to_toml().unwrap());
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
                global: GlobalConfig { time_zone: Some(UTC) }  
            }
        }).await
    }
    #[tokio::test]
    async fn deserialize_remote_storage_client_config() {        let config: Config = Config::from_toml(REMOTE_STORAGE_CONFIG_TOML).unwrap();
        assert_eq!(&config, get_remote_storage_client_config_struct().await);
    }

    #[tokio::test]
    async fn serialize_remote_storage_client_config() {
        assert_eq!(REMOTE_STORAGE_CONFIG_TOML, get_remote_storage_client_config_struct().await.to_toml().unwrap());
    }
}

