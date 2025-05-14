mod remote;

use std::{fmt::{Display, Formatter}, path::{Path, PathBuf}, str::FromStr};

pub use remote::*;

use crate::error::Error;
use serde::{
    Deserialize,
    Serialize
};

#[cfg(feature="desktop")]
use tokio::{fs::{self, File}, io::AsyncWriteExt};

pub static DEFAULT_CONFIG_FILE_NAME: &str = "dpts_client.toml";
pub static DEFAULT_CONFIG_DIR_NAME: &str = "dpts";

pub fn get_default_config_file_path() -> Result<PathBuf, Error> {
    let config_dir = dirs::config_dir().ok_or(Error::DefaultConfigDir)?;
    Ok(config_dir.join(DEFAULT_CONFIG_DIR_NAME).join(DEFAULT_CONFIG_FILE_NAME))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClientConfig {
    pub remote: ClientRemoteConfig,
}

#[cfg(feature="desktop")]
impl ClientConfig {
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
}

#[cfg(test)]
mod tests {
    use super::*;
}