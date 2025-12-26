use crate::prelude::*;
use rustix::process::{getgid, getuid};
use std::fs::write;

// mask user & group w an arbitrary one
#[inline]
pub(super) fn mask(pid: u32, uid: u32, gid: u32) -> std::io::Result<()> {
    let (cuid, cgid) = (getuid(), getgid());

    write(
        format!("/proc/{}/uid_map", pid),
        format!("0 {0} 1\n{1} {1} 1", cuid.as_raw(), uid),
    )?;
    write(format!("/proc/{}/setgroups", pid), "deny")?;
    write(
        format!("/proc/{}/gid_map", pid),
        format!("0 {0} 1\n{1} {1} 1", cgid.as_raw(), gid),
    )?;

    Ok(())
}
