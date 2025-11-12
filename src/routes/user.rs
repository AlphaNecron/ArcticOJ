use sea_orm::Iden;
use crate::model::prelude::*;
use super::prelude::*;

#[routes]
pub(crate) enum APIRoutes {
    #[get("/users", handler=get_users)]
    GetUsers,
    #[post("/users", handler=create_user)]
    CreateUser
}

async fn get_users() -> Json<Vec<user::Model>> {
    // user::Entity::find().all().await
    Json(vec!())
}

async fn create_user() -> Json<user::Model> {
    Json(user::Model {
        id: 0,
        handle: "an ba to com".to_string(),
        display_name: "hihi".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
    })
}