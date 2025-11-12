use crate::config::Config;
use axum_routes::router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod hash;
mod model;
mod prelude;
mod routes;
mod config;
mod log;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let conf = Config::load();

    let trace = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    let app = router!(routes::ArcticApp,
        custom_api = #move |route| {
            route.layer(trace.clone())
        },
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
