use config::Config as _Config;
use sea_orm::{Database, DatabaseConnection, DbBackend, DbErr, Statement};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub(crate) fn load() -> Self {
        _Config::builder()
            .add_source(config::File::with_name("arctic.corn"))
            .add_source(config::Environment::with_prefix("ARCTIC"))
            .build()
            .unwrap()
            .try_deserialize::<Config>()
            .unwrap()
    }

    pub(crate) async fn create_db_conn(&self) -> Result<DatabaseConnection, DbErr> {
        Database::connect(self.database_url.to_owned()).await
    }
}
