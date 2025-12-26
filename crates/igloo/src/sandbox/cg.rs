use crate::prelude::*;
use rustix::path::Arg;
use rustix::{fs, system};
use std::ffi;
use std::fs::{create_dir_all, exists, read_dir, remove_dir, write};

const TARGET_KERN: (u8, u8) = (5, 19);
const ROOT_SLICE: &str = "/sys/fs/cgroup/igloo.slice";
const CTRLS: &str = "+memory +pids +cpu +cpuset +io";
const CGV2_SUPER_MAGIC: ffi::c_long = 1667723888;

fn check() -> std::io::Result<()> {
    let u = system::uname();

    let ver: Vec<u8> = u
        .release()
        .to_string_lossy()
        .split('.')
        .take(2)
        .map(|x| x.parse().unwrap())
        .collect();

    assert!((ver[0], ver[1]) >= TARGET_KERN, "expect kernel >= 5.19");

    let s = fs::statfs("/sys/fs/cgroup")?;
    assert_eq!(s.f_type, CGV2_SUPER_MAGIC, "expect cgroupv2");

    Ok(())
}

fn create_cg(p: impl AsRef<std::path::Path> + std::fmt::Display) -> std::io::Result<()> {
    debug!(path = %p, "creating cg");
    create_dir_all(p)
}

fn rec_destroy_cg() -> std::io::Result<()> {
    if !exists(ROOT_SLICE).is_ok_and(|x| x) {
        return Ok(());
    }
    debug!("cleaning up prev cg");
    for e in read_dir(ROOT_SLICE)? {
        if let Ok(e) = e
            && e.metadata()?.is_dir()
        {
            let p = e.path();
            let ps = &p.to_string_lossy();
            w(ps, "cgroup.kill", "1").ok();
            remove_dir(&p).ok();
            debug!(path = ps.to_string(), "cleaning up cg")
        }
    }
    Ok(())
}

#[inline]
fn w(p: &str, f: &str, buf: &str) -> std::io::Result<()> {
    debug!(f = f, path = p, buf = buf, "write to cg");
    write(format!("{}/{}", p, f), buf)
}

pub(super) fn init() -> std::io::Result<()> {
    check()?;
    rec_destroy_cg()?;
    create_cg(ROOT_SLICE)?;
    w(ROOT_SLICE, "cgroup.subtree_control", CTRLS).expect("couldnt write to subtree_control");
    w(ROOT_SLICE, "cgroup.max.depth", "2").expect("couldnt write to max_depth");
    Ok(())
}

pub(super) fn destroy() -> std::io::Result<()> {
    debug!("destroying root cg");
    w(ROOT_SLICE, "cgroup.kill", "1").ok();
    remove_dir(ROOT_SLICE)
    // Ok(())
}

pub(super) struct CG(String);

impl Drop for CG {
    fn drop(&mut self) {
        debug!(path = self.0, "destroying child cg");
        w(&self.0, "cgroup.kill", "1").ok();
        remove_dir(&self.0).ok();
    }
}

impl CG {
    pub(super) fn new(id: impl Into<String>, cpu: impl Into<String>) -> Self {
        let p = format!("{}/{}.scope", ROOT_SLICE, id.into());
        create_cg(&p).expect("err creating sub cgroup");
        w(&p, "cpuset.cpus", &cpu.into()).expect("err setting cpu for cg");
        Self(p)
    }

    pub(super) fn fd(&self) -> rustix::io::Result<std::os::fd::OwnedFd> {
        fs::openat(
            fs::CWD,
            &self.0,
            fs::OFlags::RDONLY | fs::OFlags::DIRECTORY | fs::OFlags::CLOEXEC,
            fs::Mode::empty(),
        )
    }
}
