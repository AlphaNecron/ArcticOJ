mod auth;
mod prelude;
mod problem;
mod user;

#[derive(poem_openapi::Tags)]
pub(super) enum Tags {
    Auth,
    User,
    Problem,
}

pub(crate) fn all() -> impl poem_openapi::OpenApi {
    (
        auth::Endpoints,
        problem::Endpoints,
        user::CollectionEndpoints,
        user::ItemEndpoints,
    )
}
