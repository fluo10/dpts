//mod label;
mod category;
mod config;
mod default;
mod login;
mod user;

use category::CategoryArgs;
use config::ConfigArgs;
use default::DefaultArgs;
use login::LoginArgs;
use user::UserArgs;

use clap::{Args, CommandFactory,  Parser, Subcommand};

use std::ffi::OsString;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version=true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[command(flatten)]
    args: Option<DefaultArgs>
}

impl Cli {
    pub async fn run(self) {
        if let Some(x) = self.command {
            x.run().await
        } else if let Some(x) = self.args {
            x.run().await
        }
    }
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    Category(CategoryArgs),
    Config(ConfigArgs),
    Login(LoginArgs),
    User(UserArgs),
}

impl Command {
    pub async fn run(self) {
        todo!()
    }
}
