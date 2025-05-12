use chrono_tz::Tz;

use crate::{config::DatabaseConfig, error::Error};

pub trait GlobalConfig<T> {
    fn get_config(&self) -> Option<T>;
    fn get_or_try_init_config(&self) -> Result<T, Error>;
    fn get_database_config(&self) -> Option<DatabaseConfig>;
    fn get_time_zone(&self) -> Option<Tz>;
}