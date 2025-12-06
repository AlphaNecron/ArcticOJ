use super::cg::CG;
use super::prelude::*;
use super::{ExecOptions, isolate};
use crate::prelude::*;
use rustix::fs::mkdir;
use rustix::runtime::{EXIT_FAILURE, Fork};
use rustix::{io, pipe, process, runtime, system, thread};
use std::env::temp_dir;
use std::ffi::{CStr, CString};
use std::fs::remove_dir_all;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

enum ExecErr {
    CgErr,
}

pub struct Container {
    // opts: Options,
    id: String,
    cg: CG,
}

impl Container {
    // fn cg_reinit(&self) -> Result<(), cgroups_rs::fs::error::Error> {
    //     self.cg.delete()?;
    //     self.cg.create()?;
    //     Ok(())
    // }

    pub(super) fn new(id: impl Into<String>) -> Self {
        let _id = id.into();
        Self {
            id: _id.clone(),
            // opts,
            cg: CG::new(_id, "6"),
        }
    }

    pub fn exec(&mut self, opts: ExecOptions<impl Into<Vec<u8>>>) -> std::io::Result<()> {
        // self.cg.create();
        // defer! {
        //     self.cg.delete();
        // }
        let cargv = opts
            .argv
            .into_iter()
            .map(CString::new)
            .collect::<Result<Vec<CString>, _>>()
            .unwrap_or(vec![]);

        let cenv: Vec<CString> = vec![];

        let tmp = temp_dir().join(format!(
            "jail-{}-{}",
            self.id,
            chrono::Utc::now().timestamp_millis()
        ));

        mkdir(&tmp, 0o755.into())?;

        let p = unsafe { runtime::kernel_fork() };

        match p? {
            Fork::Child(pid) => {
                //     defer_on_unwind! {
                //     let _ = runtime::tkill(pid, runtime::Signal::KILL);
                // };

                isolate::unshare()?;

                // isolate::mask_ug(pid)?;

                system::sethostname("igloo.arctic.necron.dev".as_bytes())?;

                isolate::remount(tmp)?;

                let mut argvp: Vec<*const u8> = cargv
                    .iter()
                    .map(|s| s.as_bytes_with_nul().as_ptr())
                    .collect();
                argvp.push(ptr::null());

                let mut envp: Vec<*const u8> = cenv
                    .iter()
                    .map(|s| s.as_bytes_with_nul().as_ptr())
                    .collect();
                envp.push(ptr::null());

                let e =
                    unsafe { runtime::execve(cargv[0].as_c_str(), argvp.as_ptr(), envp.as_ptr()) };

                debug!(e=?e, "err calling execve");

                runtime::exit_group(EXIT_FAILURE);
            }
            Fork::ParentOf(pid) => {
                debug!(pid = %pid, "proc spawned");
                self.cg.bindp(pid).ok();
                if let Some((p, ws)) = process::waitpid(Some(pid), process::WaitOptions::empty())? {
                    remove_dir_all(tmp).ok();
                    debug!(ws = ?ws, pid = %p, "child proc exited");
                }
            }
        }
        Ok(())
    }
}
