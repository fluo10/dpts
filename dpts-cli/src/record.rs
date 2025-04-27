use chrono::prelude::*;
use clap::{Args, Subcommand};
use crate::Error;
use std::str::FromStr;

#[derive(Args, Clone, Debug)]
pub struct AchievementArgValues {
    pub label: String,
    pub value: i8,
}

impl FromStr for AchievementArgValues {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let strvec: Vec<&str> = s.split(':').collect();
        Ok(AchievementArgValues{
            label: strvec.get(0).unwrap().to_string(),
            value: strvec.get(1).unwrap().parse()?
        })
    }
}


#[derive(Args, Clone, Debug)]
pub struct RecordAddArgs {
    #[arg(short, long)]
    pub comment: Option<String>,
    #[arg(short, long)]
    pub time: Option<DateTime<Utc>>,
    //pub achievements: Vec<String>,
}

impl RecordAddArgs {
    pub fn run(self) -> Result<(), Error> {
       unimplemented!();
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum RecordCommand {
    Add(RecordAddArgs),
}

#[derive(Args, Clone, Debug)]
pub struct RecordArgs {
    #[command(subcommand)]
    command: RecordCommand,
}

impl RecordArgs {
    pub fn run(self) -> Result<(), Error> {
        match self.command {
            RecordCommand::Add(x) => x.run(),
        }
    }
}



