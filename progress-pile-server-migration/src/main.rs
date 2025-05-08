use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(progress_pile_server_migration::Migrator).await;
}
