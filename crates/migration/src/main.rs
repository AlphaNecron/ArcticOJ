use config::model;
use sea_orm_migration::prelude::*;

#[model]
pub struct Config {
    #[knus(child, unwrap(argument))]
    pub database_url: String,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let c: Config = config::load("config.kdl", "BLIZZARD_", None)?;
    unsafe {
        std::env::set_var("DATABASE_URL", c.database_url);
    }
    cli::run_cli(migration::Migrator).await;
    Ok(())
}
