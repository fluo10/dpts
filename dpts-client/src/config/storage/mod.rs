mod remote;

pub use remote::ClientRemoteStorageConfig;

use serde::{
    Deserialize,
    Serialize
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientStorageConfig {
    Local,
    Remote(ClientRemoteStorageConfig),
}



