use crate::executor::Executor;
use std::collections::HashMap;
use tracing::{Level, error, info, info_span};

mod executor;
mod util;

#[tokio::main]
async fn main() {
    let mut executors = HashMap::<String, Box<dyn Executor + 'static>>::new();
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    let span = info_span!("self-test");
    let _enter = span.enter();
    for _exec in executor::executors() {
        info!(exec = _exec.argv0(), "testing {}", _exec.desc().id);
        match _exec.self_test().await {
            Ok(ver) => {
                info!(ver, id = _exec.desc().id, "ok");
                executors.insert(_exec.desc().id, _exec);
            }
            Err(e) => error!(id = _exec.desc().id, "{}", e),
        }
    }
}
