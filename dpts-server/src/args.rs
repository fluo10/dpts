use crate::config::PartialServerConfig;
use clap::Parser;
use dpts_core::config::{
    PartialDatabaseConfig,
    PartialGlobalConfig,
};
use std::{
    net::IpAddr,
    path::PathBuf,
};

use crate::config::ServerConfig;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    pub server: PartialServerConfig,

    #[command(flatten)]
    pub global: PartialGlobalConfig,

    #[command(flatten)]
    pub database: PartialDatabaseConfig,

    #[arg(short, long)]
    pub config_file: Option<PathBuf>, 
}

