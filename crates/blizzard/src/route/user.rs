use super::prelude::*;

pub(super) struct CollectionEndpoints;
pub(super) struct ItemEndpoints;

#[OpenApi]
impl CollectionEndpoints {
    #[oai(path = "/users", method = "get")]
    async fn get_users(&self) -> Json<Vec<user::Model>> {
        // user::Entity::find().all().await
        Json(vec![])
    }
}

#[OpenApi(prefix_path = "/user/:id")]
impl ItemEndpoints {
    #[oai(path = "/", method = "delete")]
    async fn delete_user(&self, id: Path<String>) -> PlainText<String> {
        PlainText(id.0)
    }
}