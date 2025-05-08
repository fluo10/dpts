//mod label;
mod init;
mod record;
mod user;

pub use progress_pile_client::error;


//use label::LabelArgs;
use record::{RecordArgs,RecordAddArgs};

use error::Error;
use init::InitArgs;
use clap::{Args, CommandFactory,  Parser, Subcommand};

use std::ffi::OsString;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version=true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, Subcommand)]
enum Command {
    Init(InitArgs),
    Record(RecordArgs),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        //Some(Commands::Add(x)) => x.run(),
        Command::Init(x) => x.run().await,
        //Some(Commands::Label(x)) => x.run(),
        Command::Record(x) => x.run(),
    
    }
}
