use memfd::FileSeal;
use std::io::{Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};
use std::sync::LazyLock;

mod cg;
mod clone3;
pub(crate) mod config;
mod container;
mod ipc;
mod isolate;
mod manager;
mod mount;
mod prelude;

pub use manager::Manager;

static CRYO_MFD: LazyLock<OwnedFd> = LazyLock::new(|| {
    // TODO: de-CLOEXEC when done testing
    let fd = memfd::MemfdOptions::default()
        .close_on_exec(true)
        .allow_sealing(true)
        .create("cryo")
        .unwrap();

    let mut f = fd.as_file();
    f.write(include_bytes!(env!("CRYO_BIN"))).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();

    fd.add_seals(&[
        FileSeal::SealShrink,
        FileSeal::SealGrow,
        FileSeal::SealWrite,
    ])
    .unwrap();

    unsafe { OwnedFd::from_raw_fd(fd.into_raw_fd()) }
});
