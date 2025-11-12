use super::prelude::*;

impl ActiveModelBehavior for user::ActiveModel {}

impl user::Entity {
    pub(crate) async fn by_email_or_handle<C>(db: &C, z: &str) -> ModelResult<user::Model>
    where
        C: ConnectionTrait,
    {
        Self::find()
            .filter(
                Condition::any()
                    .add(user::Column::Handle.eq(z))
                    .add(user::Column::Email.eq(z)),
            )
            .one(db)
            .await
    }
}
