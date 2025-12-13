use serde::{Deserialize, Serialize};
use std::time::Duration;

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
    Compile(CompileArgs),
    /// FD0: cgroup
    ///
    /// FD1: input
    ///
    /// FD2: output
    Exec(ExecArgs),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Resp {
    Compile(RUsage),
    Exec(RUsage, Verdict),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RUsage {
    pub cpu_time: Duration,
    pub mem: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompileArgs {
    pub time_limit: Duration,
    pub mem_limit: u32,
    pub output_limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecArgs {
    pub f_inp: Option<String>,
    pub f_out: Option<String>,

    pub time_limit: Duration,
    pub mem_limit: u32,
    pub output_limit: u32,
}

pub fn unpack(n: u32) -> [u8; 4] {
    let mut r = [0u8; 4];
    for i in 0..4 {
        r[i] = (n >> (8 * i) & 0xff) as u8;
    }
    r
}

pub fn pack(b: [u8; 4]) -> u32 {
    let mut r = 0;
    for i in (0..4).rev() {
        r |= (b[i] as u32) << (8 * i);
    }
    r
}
