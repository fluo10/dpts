use clap::{Args, Subcommand};
use chrono_tz::Tz;
use crate::error::Error;

#[derive(Args, Clone, Debug)]
pub struct LoginArgs {
    pub endpoint: String,
    #[arg(short, long)]
    pub user_name: String,
    #[arg(short, long)]
    pub password: String,
}

impl LoginArgs {
    pub async fn run(self){
        todo!()
    }
}
