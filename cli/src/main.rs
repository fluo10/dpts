//mod label;
mod record;

//use label::LabelArgs;
use record::{RecordArgs,RecordAddArgs};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version=true)]
struct Cli {
    #[command(flatten)]
    add_args: Option<RecordAddArgs>,
    #[command(subcommand)]
    command: Option<Command>,
}
#[derive(Subcommand)]
enum Command {
    //Add(RecordAddArgs),
    //Label(LabelArgs),
    Record(RecordArgs),
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        //Some(Commands::Add(x)) => x.run(),
        //Some(Commands::Label(x)) => x.run(),
        Some(Command::Record(x)) => x.run(),
        None => {unimplemented!()},
    }
}
