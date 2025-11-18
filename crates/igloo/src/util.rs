use std::ffi::OsStr;
use std::fs::metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub(crate) fn which<T: AsRef<OsStr>>(v: T) -> Option<PathBuf> {
    which::which(v).ok()
}

pub(crate) fn is_executable<T: AsRef<Path>>(p: T) -> bool {
    metadata(p)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or_else(|_| false)
}
