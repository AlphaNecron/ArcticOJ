use super::cg::CG;
use super::isolate;
use crate::prelude::*;
use cryo::{CompileArgs, Req};
use rustix::process::{Pid, WaitId, WaitIdOptions, WaitOptions, wait, waitid, waitpid};
use rustix::stdio::{dup2_stdin, dup2_stdout};
use rustix::{
    fs::{AtFlags, mkdir},
    io::{self, Errno},
    process, runtime, stdio, system,
};
use std::env::temp_dir;
use std::ffi::{CStr, CString, c_char};
use std::fs::remove_dir_all;
use std::os::fd::{AsFd, FromRawFd, OwnedFd, RawFd};
use std::path::{Path, PathBuf};
use std::ptr;

enum ExecErr {
    CgErr,
}

pub struct Container {
    // opts: Options,
    id: String,
    root: PathBuf,
    ipc: super::ipc::Conn,
    pid: OwnedFd,
}

macro_rules! cstrv {
    ($v: expr) => {{
        let mut vp: Vec<*const u8> = $v.iter().map(|s| s.as_bytes_with_nul().as_ptr()).collect();
        vp.push(std::ptr::null());
        vp
    }};
}

impl Container {
    // TODO: impl seccomp
    // TODO: impl landlock
    // TODO: reap zomb procs
    pub fn new(id: impl Into<String>, root: impl Into<String>) -> std::io::Result<Self> {
        use rustix::net::{AddressFamily, SocketFlags, SocketType, socketpair};

        let id = id.into();
        let root = PathBuf::from(root.into());

        let conf = &crate::config::CONFIG.sandbox;

        let cenv = conf
            .env_vars
            .iter()
            .map(|e| CString::new(format!("{}={}", e.key, e.value)))
            .collect::<Result<Vec<CString>, _>>()
            .unwrap_or(vec![]);

        // write from s0 -> read from s1 and vice versa
        // s0 for par, s1 for child
        let (s0, s1) = socketpair(
            AddressFamily::UNIX,
            SocketType::SEQPACKET,
            SocketFlags::CLOEXEC,
            None,
        )?;

        use super::clone3 as c3;

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
        match unsafe { c3::clone3(&mut args) } {
            0 => {
                drop(s0);
                isolate::mask_ug()?;

                system::sethostname(id.as_bytes())?;
                system::setdomainname(conf.domain_name.as_bytes())?;

                super::mount::isolate(&conf.fs, root)?;

                let argvp: Vec<*const u8> = cstrv!(vec![CString::new(id)?]);

                let envp: Vec<*const u8> = cstrv!(cenv);

                // might do some shenanigans with stderr later :p
                dup2_stdin(s1)?;

                let e = unsafe {
                    runtime::execveat(mfd, c"", argvp.as_ptr(), envp.as_ptr(), AtFlags::EMPTY_PATH)
                };
                // debug!(e = ? e, "err calling execve");

                runtime::exit_group(1);
            }
            pid if pid > 0 => {
                drop(s1);
                // TODO: handle when pidfd = uninit
                dbg!(&args);
                debug!(%pid, "proc spawned");
            }
            e => return Err(Errno::from_raw_os_error(-e as i32).into()),
        }
        // TODO: handle when pidfd = uninit
        dbg!(pidfd);
        let pidfd = unsafe { OwnedFd::from_raw_fd(pidfd) };
        let ipc = super::ipc::Conn::new(s0);
        ipc.wait()?;
        Ok(Self {
            id,
            root,
            ipc,
            pid: pidfd,
        })
    }

    pub fn wait(&self) {
        // waitid(WaitId::PidFd(self.pid.as_fd()), WaitIdOptions::empty()).unwrap();
    }

    pub fn exec(&self) {
        let cg = CG::new(&self.id, "5");
        self.ipc
            .send(
                Req::Compile(CompileArgs {
                    time_limit: Default::default(),
                    // arbitrary vals to test ser/de
                    mem_limit: 199,
                    output_limit: 120,
                }),
                &[cg.fd().unwrap().as_fd()],
            )
            .expect("xd");
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        debug!(id = self.id, "destroying container");
        remove_dir_all(&self.root).ok();
    }
}
