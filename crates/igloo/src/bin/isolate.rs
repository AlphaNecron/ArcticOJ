use std::io::stdin;
use std::{io::BufRead, os::fd::BorrowedFd, time::Duration};
use tracing::Level;

fn main() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::DEBUG)
        .init();
    let m = igloo::sandbox::Manager::new();
    let ctn = m.create_container("uwu".into(), "6".to_string()).unwrap();
    let stdin = stdin().lock();
    for l in stdin.lines() {}
    ctn.wait();
    Ok(())
}
