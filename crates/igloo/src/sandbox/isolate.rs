use crate::prelude::*;
use rustix::fs::Mode;
use rustix::mount::{MountFlags, MountPropagationFlags, UnmountFlags, mount_change, unmount};
use rustix::process::{chdir, chroot};
use rustix::thread::LinkNameSpaceType::Mount;
use rustix::{
    fs,
    mount::mount,
    path, process,
    thread::{self, UnshareFlags},
};
use std::ffi::{CStr, CString, c_int, c_short, c_uint, c_ushort};
use std::fs::{create_dir, read_dir, write};
use std::path::{Path, PathBuf};

// src/dst - rw
struct Bind<'a>(&'a str, bool);

const UID: c_uint = 65534;
const GID: c_uint = UID;

#[inline]
pub(super) fn unshare() -> rustix::io::Result<()> {
    debug!("unsharing proc");
    unsafe {
        thread::unshare_unsafe(
            UnshareFlags::NEWNS
                | UnshareFlags::NEWPID
                | UnshareFlags::NEWUSER
                | UnshareFlags::NEWUTS
                | UnshareFlags::NEWTIME
                | UnshareFlags::NEWCGROUP,
        )
    }
}

const fn binds<'a>() -> [Bind<'a>; 10] {
    [
        Bind("/bin", false),
        Bind("/lib", false),
        Bind("/lib64", false),
        Bind("/usr", false),
        Bind("/etc/ld.so.cache", false),
        Bind("/dev/null", true),
        Bind("/dev/urandom", true),
        Bind("/dev/random", true),
        Bind("/dev/zero", true),
        Bind("/dev/full", true),
    ]
}

// mask user & group w an arbitrary one
#[inline]
pub(super) fn mask_ug(pid: thread::Pid) -> std::io::Result<()> {
    debug!(pid=%pid, "masking uid/gid to {}:{}", UID, GID);
    let p = pid.as_raw_pid();
    write(format!("/proc/{}/setgroups", p), "deny")?;

    write(format!("/proc/{}/uid_map", p), format!("{0} {0} 1", GID))?;
    write(format!("/proc/{}/gid_map", p), format!("{0} {0} 1", UID))?;

    // thread::set_thread_uid(Uid::from_raw(UID)).unwrap();
    // thread::set_thread_gid(Gid::from_raw(GID)).unwrap();

    Ok(())
}

#[inline]
pub(super) fn remount(tmp: PathBuf) -> std::io::Result<()> {
    debug!(tmp = ?tmp, "mounting root");
    mount_change(
        "/",
        MountPropagationFlags::REC | MountPropagationFlags::PRIVATE,
    )?;

    mount(&tmp, &tmp, "none", MountFlags::BIND | MountFlags::REC, None)?;

    fs::mkdir(tmp.join("etc"), 0o755.into())?;
    fs::mkdir(tmp.join("dev"), 0o755.into())?;

    for b in binds() {
        let d = tmp.join(b.0.strip_prefix("/").unwrap());
        let dir = fs::stat(b.0).is_ok_and(|x| fs::FileType::from_raw_mode(x.st_mode).is_dir());
        debug!(target = b.0, rw = ?b.1, is_dir = dir, "binding");
        let mut f = MountFlags::BIND | MountFlags::REC;
        if b.1 {
            f |= MountFlags::RDONLY;
        }
        if dir {
            fs::mkdir(&d, 0o555.into())?;
        } else {
            write(&d, [])?;
        }
        mount(b.0, &d, "none", f, None)?;
    }

    debug!("cd to tmp");
    process::chdir(&tmp)?;

    debug!("pivot_root to tmp");
    fs::mkdir("old_root", 0o755.into())?;
    process::pivot_root(".", "old_root")?;

    debug!("chroot");
    chroot("/")?;

    debug!("proc isolated");

    debug!("mounting /proc");
    fs::mkdir("/proc", 0o555.into())?;

    mount(
        "proc",
        "/proc",
        "proc",
        MountFlags::NOSUID | MountFlags::NODEV | MountFlags::NOEXEC | MountFlags::RDONLY,
        None,
    )?;

    debug!("cleaning up old root");
    unmount("old_root", UnmountFlags::DETACH)?;
    fs::rmdir("old_root")?;

    Ok(())
}
