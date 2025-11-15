use super::prelude::*;

pub(super) struct CollectionEndpoints;
pub(super) struct ItemEndpoints;

#[OpenApi(prefix_path = "/users", tag = "Tags::User")]
impl CollectionEndpoints {
    #[oai(path = "/", method = "get", operation_id = "listUsers")]
    async fn get_users(&self) -> Json<Vec<user::Model>> {
        // user::Entity::find().all().await
        Json(vec![])
    }
}

#[OpenApi(prefix_path = "/user/:id", tag = "Tags::User")]
impl ItemEndpoints {
    #[oai(path = "/", method = "delete", operation_id = "deleteUser")]
    async fn delete_user(&self, id: Path<String>) -> PlainText<String> {
        PlainText(id.0)
    }
}