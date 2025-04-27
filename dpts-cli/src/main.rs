//mod label;
mod record;

//use label::LabelArgs;
use record::{RecordArgs,RecordAddArgs};

use dpts_core::error::Error;

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

fn try_parse() -> Result<Cli, Error> {
    Ok(try_parse_from(std::env::args_os())?)
}

fn try_parse_from<I, T>(itr: I) -> Result<Cli, Error>
where I: IntoIterator<Item=T>,
T: Into<OsString> + Clone,
{
    let os_string_vec: Vec<OsString> = itr.into_iter().map(|x| Into::<OsString>::into(x)).collect();
    Cli::try_parse_from(os_string_vec.clone()).or_else(|err| match err.kind() {
        clap::error::ErrorKind::InvalidSubcommand => {
            try_parse_from(vec![OsString::from("record"), OsString::from("add")].into_iter().chain(os_string_vec.clone().into_iter()))
        },
        _ => Err(err)?, 

    })
}

fn main() -> Result<(), Error> {
    let cli = try_parse();
    match cli {
        Err(_) => Ok(Cli::command().print_help()?),
        Ok(x) => match x.command {
            //Some(Commands::Add(x)) => x.run(),
            //Some(Commands::Label(x)) => x.run(),
            Command::Record(x) => x.run(),
        }
    }


}
