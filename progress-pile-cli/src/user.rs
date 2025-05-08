use clap::{Args, Subcommand};
use crate::error::Error;
use progress_pile_client::config::Config;


#[derive(Args, Clone, Debug)]
pub struct UserArgs {
    #[command(subcommand)]
    pub command: UserCommand,
}

impl UserArgs {
    pub async fn run(self) -> Result<(), Error> {
        Config::read_from_default_toml().await?.set_global();
        match self.command {
            UserCommand::Add(x) => x.add().await,
            UserCommand::List => todo!(),
            UserCommand::Modify(x) => x.modify().await,
        }
    }
}
#[derive(Clone, Debug, Subcommand)]

enum UserCommand {
    List,
    Add(UserDataArgs),
    Modify(UserDataArgs)
}

#[derive(Args, Clone, Debug)]
pub struct UserDataArgs {
    #[arg(short, long)]
    pub user_name: String,
    #[arg(short, long)]
    pub password: String,
}

impl UserDataArgs {
    pub async fn add(self) -> Result<(), Error> {
        todo!() 
    }
    pub async fn modify(self) -> Result<(), Error> {
        todo!()
    }
}
