use clap::{Args, Subcommand};


#[derive(Args, Clone, Debug)]
pub struct CategoryArgs {
    #[command(subcommand)]
    pub command: CategoryCommand,
}

impl CategoryArgs {
    pub async fn run(self){
        self.command.run().await
    }
}
#[derive(Clone, Debug, Subcommand)]

enum CategoryCommand {
    List,
    Add,
    Modify,
}

impl CategoryCommand {
    async fn run(self) {
        todo!()
    }
}