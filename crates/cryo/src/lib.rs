pub mod clone3;

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const UID: u32 = 65534;
pub const GID: u32 = UID;

#[derive(Serialize, Deserialize, Debug)]
pub enum Verdict {
    MLE,
    TLE,
    OLE,
    RTE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Req {
    /// FD0: cgroup
    ///
    /// FD1: source file
    ///
    /// FD2: compiler output
    Compile(RLimit, Vec<String>),
    /// FD0: cgroup
    ///
    /// FD1: input
    ///
    /// FD2: output
    Exec(RLimit, IO, Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RLimit {
    pub time: Duration,
    pub mem: u32,
    pub output: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IO {
    pub f_inp: Option<String>,
    pub f_out: Option<String>,
}
