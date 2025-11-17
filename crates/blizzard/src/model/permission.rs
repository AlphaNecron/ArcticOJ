use super::prelude::*;
use sea_orm::prelude::async_trait::async_trait;

#[async_trait]
impl ActiveModelBehavior for permission::ActiveModel {
    async fn after_save<C: ConnectionTrait>(
        model: permission::Model,
        db: &C,
        insert: bool,
    ) -> Result<permission::Model, DbErr> {
        Ok(model)
    }
}
