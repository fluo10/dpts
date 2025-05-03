#[cfg(feature="client")]
mod client;
mod database;
mod error;
#[cfg(feature="server")]
mod server;

#[cfg(feature="client")]
pub use client::{
    ClientConfig,
    CLIENT_CONFIG,
};

pub use database::{
    DatabaseConfig,
    PartialDatabaseConfig,
};

pub use error::Error;

#[cfg(feature="server")]
pub use server::{
    ServerConfig,
    PartialServerConfig,
    SERVER_CONFIG,
};

use chrono_tz::{Tz, UTC};

fn get_host_time_zone_or_utc() -> Tz {
    match iana_time_zone::get_timezone() {
        Ok(x) => match x.parse(){
            Ok(x) => x,
            Err(_) => UTC
        },
        Err(_) => UTC
    }   
}