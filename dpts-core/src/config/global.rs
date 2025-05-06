use chrono_tz::Tz;
#[cfg(feature="clap")]
use clap::Args;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use crate::Error;


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GlobalConfig {
    pub time_zone: Tz,
}

impl TryFrom<PartialGlobalConfig> for GlobalConfig{
    type Error = Error;
    fn try_from(p: PartialGlobalConfig) -> Result<GlobalConfig, Self::Error> {
        Ok(GlobalConfig{
            time_zone: p.time_zone.ok_or(Error::MissingConfig("time_zone".to_string()))?,
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature="clap", derive(Args))]
pub struct PartialGlobalConfig {
    #[cfg_attr(feature="clap", arg(short, long))]

    pub time_zone: Option<Tz>,
}

pub static GLOBAL_CONFIG: OnceCell<GlobalConfig> = OnceCell::const_new();