use clap::{Args, Subcommand};
use chrono_tz::Tz;
use crate::error::Error;
use progress_pile_client::auth::try_login;
use progress_pile_client::config::{
    ClientConfig, ClientRemoteStorageConfig, ClientStorageConfig, Config, GlobalConfig, PartialGlobalConfig
};

#[derive(Args, Clone, Debug)]
pub struct InitArgs {
    #[command(subcommand)]
    pub command: InitCommand,
}

impl InitArgs {
    pub async fn run(self) -> Result<(), Error> {
        match self.command {
            InitCommand::Local(x) => x.run().await,
            InitCommand::Remote(x) => x.run().await,
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
    #[arg(short, long)]
    pub time_zone: Option<Tz>,
    #[arg(short, long)]
    pub force: bool
}

impl InitLocalArgs {
    pub async fn run(self) -> Result<(), Error> {
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
    #[arg(short, long)]
    pub force: bool,
}

impl InitRemoteArgs {
    pub async fn run(self) -> Result<(), Error> {
        let token: String = try_login(&self.user_name, &self.password, &self.endpoint)?;
        let config: Config =  Config{
            global: GlobalConfig {
                time_zone: self.time_zone.clone()
            },
            client: ClientConfig {
                storage: ClientStorageConfig::Remote(ClientRemoteStorageConfig {
                    endpoint: self.endpoint,
                    access_key: token,
                }),
            }
        };
        config.write_to_default_toml().await
    }
}
