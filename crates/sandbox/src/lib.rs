use std::time::Duration;

#[cfg(target_os = "linux")]
mod linux;

#[derive(Clone)]
pub struct Options {
    pub workdir: String,
    pub hostname: String,
    pub domainname: String,
}

pub struct ExecOptions {
    pub argv0: String,
    pub argv: Vec<String>,
    pub mem_limit: usize,
    pub output_limit: usize,
    pub time_limit: Duration,
}

pub trait Environment {
    fn new(opts: Options, id: impl Into<String>) -> Self;
    fn exec(&mut self, opts: ExecOptions);
}

pub fn init() {
    #[cfg(target_os = "linux")]
    linux::init().unwrap();
}
