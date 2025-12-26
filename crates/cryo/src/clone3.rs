// it shouldnt be here but idk where to put it, maybe a shared crate later
#![allow(dead_code)]
use std::{arch::asm, io::Error};

const __NR_CLONE3: i64 = 435;

pub const VM: u64 = 256;
pub const FS: u64 = 512;
pub const FILES: u64 = 1024;
pub const SIGHAND: u64 = 2048;
pub const PIDFD: u64 = 4096;
pub const PTRACE: u64 = 8192;
pub const VFORK: u64 = 16384;
pub const PARENT: u64 = 32768;
pub const THREAD: u64 = 65536;
pub const NEWNS: u64 = 131072;
pub const SYSVSEM: u64 = 262144;
pub const SETTLS: u64 = 524288;
pub const PARENT_SETTID: u64 = 1048576;
pub const CHILD_CLEARTID: u64 = 2097152;
pub const DETACHED: u64 = 4194304;
pub const UNTRACED: u64 = 8388608;
pub const CHILD_SETTID: u64 = 16777216;
pub const NEWCGROUP: u64 = 33554432;
pub const NEWUTS: u64 = 67108864;
pub const NEWIPC: u64 = 134217728;
pub const NEWUSER: u64 = 268435456;
pub const NEWPID: u64 = 536870912;
pub const NEWNET: u64 = 1073741824;
pub const IO: u64 = 2147483648;
pub const CLEAR_SIGHAND: u64 = 4294967296;
pub const INTO_CGROUP: u64 = 8589934592;
pub const NEWTIME: u64 = 128;

#[repr(C, align(8))]
#[derive(Default, Debug)]
pub struct RawCloneArgs {
    pub flags: u64,
    pub pidfd: u64,
    pub child_tid: u64,
    pub parent_tid: u64,
    pub exit_signal: u64,
    pub stack: u64,
    pub stack_size: u64,
    pub tls: u64,
    pub set_tid: u64,
    pub set_tid_size: u64,
    pub cgroup: u64,
}

// fork uses clone under the hood so this sounds fine xD, (Clone conflicts Clone trait so)
pub enum Fork {
    Child,
    Parent(u32),
}

pub fn clone3(a0: &mut RawCloneArgs) -> Result<Fork, Error> {
    let a1 = size_of::<RawCloneArgs>();
    // TODO: test with my android phone :yum:
    match unsafe {
        let r0: i64;
        #[cfg(target_arch = "aarch64")]
        asm!(
        "svc 0",
        in("x8") __NR_CLONE3,
        inlateout("x0") a0 => r0,
        in("x1") a1,
        options(nostack, preserves_flags)
        );
        #[cfg(target_arch = "x86_64")]
        asm!(
        "syscall",
        inlateout("rax") __NR_CLONE3 => r0,
        in("rdi") a0,
        in("rsi") a1,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
        );
        r0
    } {
        0 => Ok(Fork::Child),
        pid if pid > 0 => Ok(Fork::Parent(pid as u32)),
        // casting to i32 should work, as no one would fucking spawn 2^63 procs (at least max_proc on my machine is 2^22)
        eno => Err(Error::from_raw_os_error((-eno) as i32)),
    }
}
