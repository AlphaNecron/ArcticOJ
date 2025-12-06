use clap::{Parser, Subcommand};
use dashmap::DashMap;
use igloo::sandbox;
use tracing::{Level, error, info, info_span};

#[derive(Subcommand)]
enum Commands {
    GenExecPaths { out: Option<String> },
}

// const EXECUTORS: Arc<DashMap<&str, &'static Executor>> = Arc::new(DashMap::new());

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    {
        let span = info_span!("self-test");
        let _enter = span.enter();
        //     for mut ex in executor::available() {
        //         match ex.self_test().await {
        //             Some(e) => error!(
        //                 id = ex.id,
        //                 ty = ex.ty,
        //                 exec_path = ex.exec_path,
        //                 err = e,
        //                 "invalid executor, skipping"
        //             ),
        //         }
        //     }
    }
}
