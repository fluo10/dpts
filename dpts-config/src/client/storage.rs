mod local;
mod remote;

pub use local::{
    ClientLocalStorageConfig,
    PartialClientLocalStorageConfig,
};

pub use remote::{
    ClientRemoteStorageConfig,
    PartialClientRemoteStorageConfig,
};

use serde::{
    Deserialize,
    Serialize
};

pub enum ClientStorageConfig {
    Local(ClientLocalStorageConfig),
    Remote(ClientRemoteStorageConfig),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialClientStorageConfig {
    Local(PartialClientLocalStorageConfig),
    Remote(PartialClientRemoteStorageConfig),
}