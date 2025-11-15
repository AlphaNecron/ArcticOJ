use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) conn: DatabaseConnection,
}