use std::ffi::CString;
use std::time::Duration;

mod cg;
mod container;
mod isolate;
mod manager;
mod prelude;

#[derive(Clone)]
pub struct Options {
    pub workdir: String,
    pub hostname: String,
    pub domainname: String,
}

pub struct ExecOptions<T: Into<Vec<u8>>> {
    pub argv: Vec<T>,
    pub mem_limit: usize,
    pub output_limit: usize,
    pub time_limit: Duration,
}

pub use manager::Manager;
