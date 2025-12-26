use super::cg::CG;
use crate::prelude::*;
use cryo::Req;
use rustix::process::{
    Pid, WaitId, WaitIdOptions, WaitOptions, getgid, getuid, wait, waitid, waitpid,
};
use rustix::runtime::exit_group;
use rustix::stdio::{dup2_stdin, dup2_stdout};
use rustix::{
    fs::{AtFlags, mkdir},
    io::{self, Errno},
    process, runtime, stdio, system,
};
use std::env::temp_dir;
use std::ffi::{CStr, CString, c_char};
use std::fs::remove_dir_all;
use std::os::fd::{AsFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use std::path::{Path, PathBuf};
use std::ptr;

enum ExecErr {
    CgErr,
}

pub struct Container {
    id: String,
    root: PathBuf,
    ipc: super::ipc::Conn,
    pid: OwnedFd,
    cpu: String,
}

macro_rules! cstrv {
    ($v: expr) => {{
        let mut vp: Vec<*const u8> = $v.iter().map(|s| s.as_bytes_with_nul().as_ptr()).collect();
        vp.push(std::ptr::null());
        vp
    }};
}

impl Container {
    // TODO: reap zomb procs
    pub fn new(id: String, root: String, cpu: String) -> std::io::Result<Self> {
        use rustix::net::{AddressFamily, SocketFlags, SocketType, socketpair};

        let root = PathBuf::from(root);

        let conf = &crate::config::CONFIG.sandbox;

        let cenv = conf
            .env_vars
            .iter()
            .map(|e| CString::new(format!("{}={}", e.key, e.value)))
            .collect::<Result<Vec<_>, _>>()?;

        // write from s0 -> read from s1 and vice versa
        // s0 for par, s1 for child
        let (s0, s1) = socketpair(
            AddressFamily::UNIX,
            SocketType::SEQPACKET,
            SocketFlags::CLOEXEC,
            None,
        )?;

        use cryo::clone3 as c3;

        let mut pidfd: i32 = -1;

        let mut args = c3::RawCloneArgs {
            flags: c3::NEWNS
                | c3::NEWPID
                | c3::NEWUTS
                | c3::NEWIPC
                | c3::NEWNET
                | c3::NEWTIME
                | c3::NEWUSER
                | c3::PIDFD,
            // SIGCHLD
            exit_signal: 17,
            pidfd: &mut pidfd as *mut i32 as u64,
            ..Default::default()
        };

        let mfd = &*super::CRYO_MFD;

        let (cuid, cgid) = (getuid(), getgid());

        match c3::clone3(&mut args)? {
            c3::Fork::Child => {
                drop(s0);

                let e = {
                    super::creds::mask(cuid.as_raw(), cgid.as_raw(), cryo::UID, cryo::GID)?;

                    system::sethostname(id.as_bytes())?;
                    system::setdomainname(conf.domain_name.as_bytes())?;

                    super::fs::isolate(&conf.fs, root)?;

                    // vec for scalability later :c
                    let argv: Vec<*const u8> =
                        cstrv!([CString::new(conf.fs.wd.as_str())?, CString::new(id)?]);

                    let envp: Vec<*const u8> = cstrv!(cenv);

                    // might do some shenanigans with stderr later :p
                    dup2_stdin(s1)?;

                    unsafe {
                        runtime::execveat(
                            mfd,
                            c"",
                            argv.as_ptr(),
                            envp.as_ptr(),
                            AtFlags::EMPTY_PATH,
                        )
                    }
                };
                debug!(?e, "err spawning zygote");

                exit_group(1);
            }
            c3::Fork::Parent(pid) => {
                drop(s1);
                debug!(%pid, "proc spawned");
            }
        }
        // TODO: handle when pidfd = uninit
        let pidfd = unsafe { OwnedFd::from_raw_fd(pidfd) };
        let ipc = super::ipc::Conn::new(s0);
        ipc.wait()?;
        Ok(Self {
            id,
            root,
            ipc,
            cpu,
            pid: pidfd,
        })
    }

    pub fn wait(&self) {
        waitid(WaitId::PidFd(self.pid.as_fd()), WaitIdOptions::EXITED).unwrap();
    }

    pub fn exec(&self, req: cryo::Req, fds: &[BorrowedFd]) -> std::io::Result<i32> {
        let cg = CG::new(&self.id, &self.cpu);
        // assuming len(fds) is ALWAYS 3
        // TODO: change this to use vec if using more than 3 fds
        let cg_fd = cg.fd()?;
        let fds = [cg_fd.as_fd(), fds[0], fds[1]];
        self.ipc.send(req, &fds)
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        debug!(id = self.id, "destroying container");
        remove_dir_all(&self.root).ok();
    }
}
