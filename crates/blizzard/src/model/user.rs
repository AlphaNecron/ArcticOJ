use super::prelude::*;
use crate::hash;

impl ActiveModelBehavior for user::ActiveModel {}

impl user::Entity {
    pub(crate) fn find_by_email_or_handle(z: &str) -> Select<user::Entity> {
        Self::find().filter(
            Condition::any()
                .add(user::Column::Handle.eq(z))
                .add(user::Column::Email.eq(z)),
        )
    }
}

impl user::Model {
    pub(crate) fn verify_pwd(&self, pwd: &str) -> bool {
        hash::verify_pwd(&self.password, pwd)
    }
}
