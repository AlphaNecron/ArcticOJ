use rustix::fs::OFlags;
use rustix::mount::{
    MountFlags, MountPropagationFlags, UnmountFlags, mount_change, mount_remount, unmount,
};
use rustix::process::{chdir, chroot};
use rustix::{fs, io, mount::mount, process};
use std::ffi::CString;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

#[inline]
// TODO: chown/chgrp ext dirs / files to pseudo-root instead of `igloo:igloo`
pub(super) fn isolate(fs: &super::config::FS, tmp: PathBuf) -> std::io::Result<()> {
    // debug!(tmp = ?tmp, "remounting root");
    mount_change(
        "/",
        MountPropagationFlags::REC | MountPropagationFlags::PRIVATE,
    )?;

    mount(&tmp, &tmp, "", MountFlags::BIND | MountFlags::REC, None)?;

    for b in &fs.dir_binds {
        // debug!(
        //     rw = b.rw.unwrap_or(false),
        //     exec = b.exec.unwrap_or(false),
        //     path = b.path,
        //     "binding dir {:?}",
        //     b.opts(),
        // );
        let d = tmp.join(b.path.strip_prefix("/").unwrap());
        create_dir_all(&d)?;
        mount(&b.path, &d, "", MountFlags::BIND | MountFlags::REC, None)?;
        if !b.flags().is_empty() {
            mount_remount(d, MountFlags::BIND | MountFlags::REC | b.flags(), b.opts())?;
        }
    }

    for b in &fs.file_binds {
        // debug!(
        //     rw = b.rw.unwrap_or(false),
        //     exec = b.exec.unwrap_or(false),
        //     path = b.path,
        //     "binding file {:?}",
        //     b.opts()
        // );
        let d = tmp.join(b.path.strip_prefix("/").unwrap());
        if let Some(par) = d.parent() {
            create_dir_all(par)?;
        }
        write(&d, [])?;
        mount(&b.path, &d, "", MountFlags::BIND, None)?;
        if !b.flags().is_empty() {
            mount_remount(d, MountFlags::BIND | b.flags(), b.opts())?;
        }
    }

    // debug!("cd to tmp");
    process::chdir(&tmp)?;

    // debug!("pivot_root to tmp");
    fs::mkdir("old_root", 0o755.into())?;
    process::pivot_root(".", "old_root")?;

    for m in &fs.mounts {
        // debug!(opts = m.opts, path = m.path, "mounting",);
        create_dir_all(&m.path)?;
        let opts = CString::new(m.opts.clone())?;
        mount(
            &m.name,
            &m.path,
            &m.ty,
            MountFlags::empty(),
            opts.as_c_str(),
        )?;
    }

    // debug!("chroot");
    chroot("/")?;

    // debug!("proc isolated");

    // debug!("mounting /proc");
    fs::mkdir("/proc", 0o555.into())?;

    mount(
        "proc",
        "/proc",
        "proc",
        MountFlags::NOSUID | MountFlags::NODEV | MountFlags::NOEXEC | MountFlags::RDONLY,
        None,
    )?;

    for m in &fs.masks {
        // debug!(path = m, "masking file");
        // if let Some(par) = &m.parent() {
        //     create_dir_all(par)?;
        // }
        write(m, [])?;
        mount("/dev/null", m, "", MountFlags::BIND, None)?;
    }

    // placing this inside pivot_root seems safer, I accidentally deleted /etc/passwd during dev tho :sob:
    for f in &fs.files {
        // debug!(path = f.path, buf = f.buf, "creating file (mode={:#05o})", f.perm);
        let fd = fs::openat(
            fs::CWD,
            &f.path,
            OFlags::CREATE | OFlags::WRONLY | OFlags::TRUNC | OFlags::CLOEXEC,
            fs::Mode::from_raw_mode(f.perm as u32),
        )?;
        io::write(&fd, f.buf.as_bytes())?;
    }

    // debug!("cleaning up old root");
    unmount("old_root", UnmountFlags::DETACH)?;
    fs::rmdir("old_root")?;

    chdir(&fs.wd)?;

    Ok(())
}
