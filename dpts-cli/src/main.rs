//mod label;
pub mod error;
mod record;


//use label::LabelArgs;
use record::{RecordArgs,RecordAddArgs};

use error::Error;

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
    Record(RecordArgs),
}


fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        //Some(Commands::Add(x)) => x.run(),
        //Some(Commands::Label(x)) => x.run(),
        Command::Record(x) => x.run(),
    
    }


}
