use igloo::sandbox::ExecOptions;
use std::ffi::CString;
use tracing::Level;

fn main() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    let m = igloo::sandbox::Manager::new();
    let mut env = m.create_container("uwu");
    env.exec(ExecOptions {
        argv: vec![
            "/bin/sh",
            // "-c\0".to_string(),
            // "echo 'Hi'\0".to_string(),
            // "/usr/bin/python3".to_string(),
            //                                       "/data/Dev/py/test.py".to_string(),
        ],
        mem_limit: 0,
        output_limit: 0,
        time_limit: Default::default(),
    })
    .unwrap();
    Ok(())
}
