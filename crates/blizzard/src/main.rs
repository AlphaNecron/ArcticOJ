use crate::prelude::*;
use crate::state::AppState;
use miette::{IntoDiagnostic, WrapErr};
use poem::listener::{Listener, TcpListener, UnixListener};
use poem::middleware::{AddData, CookieJarManager, RequestId, Tracing};
use poem::{EndpointExt, Route, Server};
use poem_grants::GrantsMiddleware;
use poem_openapi::OpenApiService;
use tracing::Level;

pub mod config;
mod db;
mod error;
mod extractor;
mod hash;
mod middleware;
mod model;
mod payload;
mod prelude;
mod rbac;
mod route;
mod scalar;
mod state;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();

    miette::set_panic_hook();

    let conf = config::Config::load()?;
    info!("config loaded");

    let db = db::from(&conf.database_url)
        .await
        .into_diagnostic()
        .wrap_err("err connecting to db")?;

    let mut app = OpenApiService::new(
        route::all(),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .summary("Just another code jury platform.");

    if conf.listener.protocol == config::Protocol::Tcp {
        app = app.server(format!("http://{}", conf.listener.path))
    }

    let scalar = scalar::endpoint();

    let spec = app.spec_endpoint();

    Server::new(match conf.listener.protocol {
        #[cfg(target_family = "unix")]
        config::Protocol::Unix => UnixListener::bind(conf.listener.path.to_owned()).boxed(),

        _ => TcpListener::bind(conf.listener.path.to_owned()).boxed(),
    })
    .run(
        Route::new()
            .nest(
                "/",
                app.with(RequestId::default())
                    .with(AddData::new(AppState { conn: db }))
                    .with(CookieJarManager::new())
                    .with_if(cfg!(debug_assertions), Tracing)
                    .with(GrantsMiddleware::with_extractor(extractor::grants::extract)),
            )
            .nest("/openapi.json", spec)
            .nest("/docs", scalar),
    )
    .await
    .into_diagnostic()?;
    Ok(())
}
