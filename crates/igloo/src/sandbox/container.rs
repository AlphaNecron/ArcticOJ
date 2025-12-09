use super::cg::CG;
use super::{ExecOptions, isolate};
use crate::prelude::*;
use rustix::fs::mkdir;
use rustix::io::Errno;
use rustix::runtime::EXIT_FAILURE;
use rustix::{process, runtime, system};
use std::env::temp_dir;
use std::ffi::CString;
use std::fs::remove_dir_all;
use std::ptr;

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
        println!("{:?}", *super::CONTAINER_CONF);
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

        let cenv = super::CONTAINER_CONF
            .env_vars
            .iter()
            .map(|e| CString::new(format!("{}={}", e.key, e.value)))
            .collect::<Result<Vec<CString>, _>>()
            .unwrap_or(vec![]);

        let tmp = temp_dir().join(format!(
            "jail-{}-{}",
            self.id,
            chrono::Utc::now().timestamp_millis()
        ));

        mkdir(&tmp, 0o755.into())?;

        match unsafe {
            clone3::clone3_system_call(
                &clone3::Clone3::default()
                    .flag_into_cgroup(&self.cg.fd()?)
                    .flag_newns()
                    .flag_newpid()
                    .flag_newuts()
                    .flag_newipc()
                    .flag_newnet()
                    .flag_newtime()
                    .flag_newuser()
                    .as_clone_args(),
            )
        } {
            0 => {
                //     defer_on_unwind! {
                //     let _ = runtime::tkill(pid, runtime::Signal::KILL);
                // };

                // isolate::unshare()?;

                isolate::mask_ug()?;

                system::sethostname(super::CONTAINER_CONF.hostname.as_bytes())?;

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
            pid if pid > 0 => {
                debug!(pid = %pid, "proc spawned");
                let p = process::Pid::from_raw(pid as i32).unwrap();
                self.cg.bindp(p).ok();
                if let Some((p, ws)) = process::waitpid(Some(p), process::WaitOptions::empty())? {
                    remove_dir_all(tmp).ok();
                    debug!(ws = ?ws, pid = %p, "child proc exited");
                }
            }
            e => return Err(Errno::from_raw_os_error(e as i32).into()),
        }
        Ok(())
    }
}
