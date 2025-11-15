use poem_openapi::OpenApi;

mod auth;
mod prelude;
mod problem;
mod user;

pub(crate) fn all() -> impl OpenApi {
    (
        auth::Endpoints,
        problem::Endpoints,
        user::CollectionEndpoints,
        user::ItemEndpoints,
    )
}
