use super::prelude::*;
use crate::model::prelude::*;

use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DbBackend, DbErr, EntityName, EntityRegistry,
    SchemaBuilder,
};

pub async fn from(src: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new(src);
    opts.sqlx_logging(false);
    let conn = Database::connect(opts).await?;
    info!(
        backend = match conn.get_database_backend() {
            DbBackend::Postgres => "postgres",
            DbBackend::Sqlite => "sqlite",
            _ => "unknown",
        },
        "db connected"
    );
    // TODO: might add migration & rbac init to migrator-cli later
    if cfg!(debug_assertions) {
        conn.get_schema_registry(concat!(env!("CARGO_CRATE_NAME"), "::model::entity::*"))
            .sync(&conn)
            .await?;
    }
    Ok(conn)
}
