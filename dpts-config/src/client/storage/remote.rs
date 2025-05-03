use chrono_tz::{Tz, UTC};
use crate::{get_host_time_zone_or_utc, Error};
use serde::{
    Deserialize,
    Serialize
};

use std::{
    str::FromStr,
    net::IpAddr
};

pub struct ClientRemoteStorageConfig {
    pub endpoint: String,
    pub access_key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PartialClientRemoteStorageConfig {
    pub endpoint: Option<String>,
    pub access_key: Option<String>,
}