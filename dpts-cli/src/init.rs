use clap::{Args, Subcommand};
use chrono_tz::Tz;
use crate::error::Error;
use dpts_client::auth::try_login;
use dpts_client::config::{
    Config,
    ClientConfig,
    ClientRemoteStorageConfig,
    ClientStorageConfig,
    PartialGlobalConfig
};

#[derive(Args, Clone, Debug)]
pub struct InitArgs {
    #[command(subcommand)]
    pub command: InitCommand,
}

impl InitArgs {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            InitCommand::Local(x) => x.run(),
            InitCommand::Remote(x) => x.run(),
        }
    }
}
#[derive(Clone, Debug, Subcommand)]

enum InitCommand {
    Local(InitLocalArgs),
    Remote(InitRemoteArgs),
}

#[derive(Args, Clone, Debug)]
pub struct InitLocalArgs {
    pub time_zone: Option<Tz>,
}

impl InitLocalArgs {
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Args, Clone, Debug)]
pub struct InitRemoteArgs {
    pub endpoint: String,
    #[arg(short, long)]
    pub user_name: String,
    #[arg(short, long)]
    pub password: String,
    #[arg(short, long)]
    pub time_zone: Option<Tz>,
}

impl InitRemoteArgs {
    pub fn run(self) -> Result<(), Error> {
        let token: String = try_login(&self.user_name, &self.password, &self.endpoint)?;
        let config: Config =  Config{
            global: PartialGlobalConfig {
                time_zone: self.time_zone.clone()
            },
            client: ClientConfig {
                storage: ClientStorageConfig::Remote(ClientRemoteStorageConfig {
                    endpoint: self.endpoint,
                    access_key: token,
                }),
            }
        };
        todo!() // Write config
    }
}
