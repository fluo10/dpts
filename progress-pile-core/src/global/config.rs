use chrono_tz::Tz;

use crate::error::Error;

pub trait GlobalConfig<T> {
    fn get_config(&self) -> Option<T>;
    fn get_or_try_init_config(&self) -> Result<T, Error>;
    fn get_or_try_init_config_from_file(&self) -> Result<T, Error>;
    fn get_or_try_init_config_from_str(&self) -> Result<T, Error>;
    fn get_time_zone(&self) -> Option<Tz>;
}