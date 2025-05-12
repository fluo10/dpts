use chrono_tz::Tz;

use crate::{config::DatabaseConfig, error::Error};

pub trait GlobalConfigTrait<T> {
    fn get(&self) -> Option<T>;
    fn get_or_try_init(&self) -> Result<T, Error>;
    fn get_database_config(&self) -> Option<DatabaseConfig>;
    fn get_time_zone(&self) -> Option<Tz>; 
}