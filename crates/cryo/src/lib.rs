use serde::{Deserialize, Serialize};
use std::fs::File;
use std::os::fd::{BorrowedFd, OwnedFd};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub enum Job {
    Compile,
    Init,
    Exec(ExecArgs),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecArgs {
    time_limit: Duration,
    mem_limit: u32,
    output_limit: u32,
}
