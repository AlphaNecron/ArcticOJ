use crate::prelude::*;
use std::fs::write;

// mask user & group w an arbitrary one
#[inline]
pub(super) fn mask(cuid: u32, cgid: u32, uid: u32, gid: u32) -> std::io::Result<()> {
    debug!("masking uid/gid to {}:{}", uid, gid);

    write(
        "/proc/self/uid_map",
        format!("0 {0} 1\n{1} {1} 1", cuid, uid),
    )?;
    write("/proc/self/setgroups", "deny")?;
    write(
        "/proc/self/gid_map",
        format!("0 {0} 1\n{1} {1} 1", cgid, gid),
    )?;

    Ok(())
}
