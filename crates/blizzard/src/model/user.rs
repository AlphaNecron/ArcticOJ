use super::prelude::*;
use crate::hash;
use cuid2::CuidConstructor;
use pasetors::claims::Claims;
use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::public;
use pasetors::version4::V4;

const CUID: CuidConstructor = CuidConstructor::new().with_length(32);

#[async_trait::async_trait]
impl ActiveModelBehavior for user::ActiveModel {
    async fn before_save<C: ConnectionTrait>(
        mut self,
        _db: &C,
        insert: bool,
    ) -> Result<Self, DbErr> {
        if insert {
            self.password = Set(hash::hash_pwd(&self.password.take().unwrap()));
            self.uid = Set(CUID.create_id());
        }
        Ok(self)
    }
}

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

    pub(crate) fn create_token(&self) -> Result<String, pasetors::errors::Error> {
        let mut claims = Claims::new()?;
        // should i use id directly?
        claims.add_additional("uid", self.uid.clone())?;
        let kp = AsymmetricKeyPair::<V4>::generate()?;
        public::sign(&kp.secret, &claims, None, Some(b"implicit assertion"))
    }
}
