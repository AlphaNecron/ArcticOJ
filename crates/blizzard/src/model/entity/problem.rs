use super::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Object)]
#[oai(rename = "Problem")]
#[sea_orm(table_name = "problems")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[oai(read_only)]
    pub id: i32,

    #[sea_orm(unique)]
    pub code: String,

    #[sea_orm(default_value = 1f32)]
    pub time_limit: f32,

    #[sea_orm(default_value = 256)]
    pub memory_limit: i16,

    #[sea_orm(default_value = 1024)]
    pub output_limit: i16,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    #[oai(read_only)]
    pub created_at: DateTimeUtc,
}
