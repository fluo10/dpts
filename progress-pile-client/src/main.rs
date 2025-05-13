use clap::Parser;
use progress_pile_client::cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.run().await
}
