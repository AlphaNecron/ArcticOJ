use super::prelude::*;
use super::syscall;
use crate::{Environment, ExecOptions, Options};
use cgroups_rs::fs::cgroup_builder::CgroupBuilder;
use cgroups_rs::fs::{Cgroup, hierarchies};

const FLAGS: libc::c_int = libc::CLONE_NEWNS
    | libc::CLONE_NEWPID
    | libc::CLONE_NEWUSER
    | libc::CLONE_NEWUTS
    | libc::CLONE_NEWCGROUP
    | libc::SIGCHLD;

fn controllers() -> Vec<String> {
    ["cpu", "cpuset", "pid"].map(ToString::to_string).to_vec()
}

pub(crate) struct LinuxEnv {
    opts: Options,
    cg: Box<Cgroup>,
}

impl LinuxEnv {
    fn cg_reinit(&self) -> Result<(), cgroups_rs::fs::error::Error> {
        self.cg.delete()?;
        self.cg.create()?;
        Ok(())
    }
}

impl Environment for LinuxEnv {
    fn new(opts: Options, id: impl Into<String>) -> Self {
        Self {
            opts,
            cg: Box::new(
                CgroupBuilder::new(&format!("igloo.slice/{}.scope", id.into()))
                    .set_specified_controllers(controllers())
                    .build(hierarchies::auto())
                    .unwrap(),
            ),
        }
    }

    fn exec(&mut self, opts: ExecOptions) {
        self.cg.create();
        defer! {
            self.cg.delete();
        }
        syscall::clone3(libc::clone_args {
            flags: 0,
            pidfd: 0,
            child_tid: 0,
            parent_tid: 0,
            exit_signal: 0,
            stack: 0,
            stack_size: 0,
            tls: 0,
            set_tid: 0,
            set_tid_size: 0,
            cgroup: 0,
        });
    }
}
