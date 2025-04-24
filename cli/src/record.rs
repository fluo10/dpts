use achievement_counter_core::Record;
use chrono::prelude::*;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct RecordAddArgs {
    pub label: String,
    #[arg(short, long)]
    pub comment: Option<String>,
    #[arg(short, long)]
    pub time: Option<DateTime<Utc>>,
    #[arg(default_value_t = 1)]
    pub count: u8,
}
impl RecordAddArgs {
    pub fn run(&self) {
       unimplemented!();
    }
}

#[derive(Subcommand)]
pub enum RecordCommand {
    Add(RecordAddArgs),
}

#[derive(Args)]
pub struct RecordArgs {
    #[command(subcommand)]
    command: RecordCommand,
}

impl RecordArgs {
    pub fn run(&self) {
        match &self.command {
            RecordCommand::Add(x) => x.run(),
        }
    }
}



