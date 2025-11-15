use super::prelude::*;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbBackend, DbErr};

pub async fn from(src: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(src);
    opts.sqlx_logging(false);
    let conn = Database::connect(opts).await?;
    info!(backend=match conn.get_database_backend() {
        DbBackend::Postgres => "postgres",
        DbBackend::Sqlite => "sqlite",
        _ => "unknown",
    }, "db connected");
    if cfg!(debug_assertions) {
        conn.get_schema_registry("arctic::model::entity::*").sync(&conn).await?;
    }
    Migrator::up(&conn, None).await?;
    Ok(conn)
}
