mod client;
mod error;
mod server;
mod user;

pub use error::Error;
pub use server::{
    ServerConfig,
    PartialServerConfig,
    SERVER_CONFIG,
};

pub use client::{
    ClientConfig,
    PartialClientConfig,
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