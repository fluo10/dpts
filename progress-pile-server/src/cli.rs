use crate::config::PartialServerConfig;
use clap::Parser;

use std::{
    net::IpAddr,
    path::PathBuf,
};

use crate::config::ServerConfig;

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub config_file: Option<PathBuf>, 
}

