pub(super) use sea_orm::{
    ActiveModelBehavior, ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, Condition, QueryFilter,
};
pub(super) type ModelResult<T> = Result<Option<T>, sea_orm::DbErr>;
pub(super) use super::mixin::*;
pub(crate) use super::entity::*;
