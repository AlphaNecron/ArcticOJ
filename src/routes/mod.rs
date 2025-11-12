use axum_routes::routes;

mod prelude;
mod user;

#[routes]
pub(crate) enum ArcticApp {
    #[nest("/api")]
    Api(user::APIRoutes),
}
