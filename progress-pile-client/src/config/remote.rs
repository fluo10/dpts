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
#[serde(rename_all = "snake_case")]
pub struct ClientRemoteConfig {
    pub endpoint: String,
    pub access_key: String,
}