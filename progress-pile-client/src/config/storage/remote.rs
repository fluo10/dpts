use crate::error::Error;
use serde::{
    Deserialize,
    Serialize
};

use std::{
    str::FromStr,
    net::IpAddr
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]

pub struct ClientRemoteStorageConfig {
    pub endpoint: String,
    pub access_key: String,
}