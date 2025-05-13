use clap::{Args, Subcommand};


#[derive(Args, Clone, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

impl ConfigArgs {
    pub async fn run(self){
        self.command.run().await
    }
}
#[derive(Clone, Debug, Subcommand)]

enum ConfigCommand {
    Show,
    Modify,
}

impl ConfigCommand {
    async fn run(self) {
        todo!()
    }
}