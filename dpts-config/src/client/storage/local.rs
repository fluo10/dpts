use std::path::PathBuf;
use serde::{
    Deserialize,
    Serialize
};

pub struct ClientLocalStorageConfig {
    pub database_path: PathBuf
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialClientLocalStorageConfig {
    pub database_path: Option<PathBuf>,
}

