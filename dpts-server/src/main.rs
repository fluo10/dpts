use dpts_server::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
