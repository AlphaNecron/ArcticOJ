use crate::prelude::*;
use crate::state::AppState;
use poem::listener::{Listener, TcpListener, UnixListener};
use poem::middleware::{AddData, RequestId, Tracing};
use poem::{EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use tracing::Level;

mod config;
mod db;
mod error;
mod extractor;
mod hash;
mod log;
mod middleware;
mod model;
mod payload;
mod prelude;
mod route;
mod scalar;
mod state;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // tracing::subscriber::set_global_default(Registry::default().with(JsonStorageLayer).with(
    //     BunyanFormattingLayer::new("blizzard".into(), std::io::stdout),
    // )).unwrap();
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();

    let conf = config::load();
    info!("config loaded");

    let db = db::from(&conf.database_url).await.unwrap_or_else(|e| {
        error!(err = e.to_string(), "err connecting to db");
        std::process::exit(1);
    });

    let app = OpenApiService::new(
        route::all(),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .summary("Just another code jury platform.")
    .server("http://localhost:2999");

    let scalar = scalar::endpoint();

    let spec = app.spec_endpoint();

    let mut sugared_app = app
        .with(RequestId::default())
        .with(AddData::new(AppState { conn: db }))
        .boxed();
    if cfg!(debug_assertions) {
        sugared_app = sugared_app.with(Tracing).boxed();
    }

    Server::new(match conf.listener.protocol {
        #[cfg(target_family = "unix")]
        config::Protocol::Unix => UnixListener::bind(conf.listener.addr).boxed(),
        _ => TcpListener::bind(conf.listener.addr).boxed(),
    })
    .run(
        Route::new()
            .nest("/", sugared_app)
            .nest("/openapi.json", spec)
            .nest("/docs", scalar),
    )
    .await
}
