use super::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i16,

    #[sea_orm(unique_key = "code")]
    pub resource: String,

    #[sea_orm(unique_key = "code")]
    pub operation: String,
}
