use crate::runtime::Runtime;
use std::collections::HashMap;
use tracing::{Level, error, info, info_span};

mod runtime;
mod util;

#[tokio::main]
async fn main() {
    let mut runtimes = HashMap::<String, Box<dyn Runtime + 'static>>::new();
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    let span = info_span!("self-test");
    let _enter = span.enter();
    for rt in runtime::runtimes() {
        info!(exec = rt.argv0(), "testing {}", rt.desc().id);
        match rt.self_test().await {
            Ok(ver) => {
                info!(ver, id = rt.desc().id, "ok");
                runtimes.insert(rt.desc().id, rt);
            }
            Err(e) => error!(id = rt.desc().id, "{}", e),
        }
    }
}
