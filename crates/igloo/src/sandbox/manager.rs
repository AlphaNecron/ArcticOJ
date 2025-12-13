use super::container::Container;
use super::{CRYO_MFD, cg};
use crate::config::CONFIG;
use rustix::fs::mkdir;
use rustix::io::close;
use rustix::path::Arg;
use std::fs::create_dir_all;
use std::os::fd::AsRawFd;
use std::path::Path;

// dunno what to name here, Pool is not a bad choice but `igloo` should be handling it, not `sandbox`...
pub struct Manager;

#[allow(clippy::new_without_default)]
impl Manager {
    pub fn new() -> Self {
        cg::init().expect("err init cgroup tree");
        create_dir_all(&CONFIG.sandbox.fs.root).expect("err creating root");
        Self
    }

    pub fn create_container(&self, id: String) -> std::io::Result<Container> {
        let tmp = Path::new(&CONFIG.sandbox.fs.root).join(format!(
            "{}_{}",
            id,
            chrono::Utc::now().timestamp_millis()
        ));

        mkdir(&tmp, 0o755.into()).expect("err creating tmpdir");
        Container::new(id, tmp.to_string_lossy())
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        cg::destroy().ok();
        unsafe { close(CRYO_MFD.as_raw_fd()) };
    }
}
