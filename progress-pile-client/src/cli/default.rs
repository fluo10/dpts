use clap::{Args, Subcommand};


#[derive(Args, Clone, Debug)]
pub struct DefaultArgs {
    #[arg(short, long)]
    pub note: String,
    pub category: String,
    pub quantity: i32,
}

impl DefaultArgs {
    pub async fn run (self) {
        todo!()
    }
}