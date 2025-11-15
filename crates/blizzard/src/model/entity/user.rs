use super::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize, Object)]
#[oai(rename = "User")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[oai(read_only)]
    pub id: i32,

    #[sea_orm(unique)]
    #[oai(read_only)]
    pub handle: String,

    #[sea_orm(nullable)]
    pub display_name: Option<String>,

    #[sea_orm(unique)]
    pub email: String,

    #[oai(skip)]
    pub password: String,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    #[oai(read_only)]
    pub created_at: DateTimeUtc,

    #[sea_orm(unique, nullable, indexed)]
    pub api_key: Option<String>,

    #[sea_orm(default_value = 0)]
    #[oai(read_only)]
    pub rating: i16,

    #[sea_orm(nullable)]
    pub bio: Option<String>,
}
