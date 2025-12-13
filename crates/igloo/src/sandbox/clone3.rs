use std::arch::asm;

const __NR_CLONE3: i64 = 435;

pub(super) const VM: u64 = 256;
pub(super) const FS: u64 = 512;
pub(super) const FILES: u64 = 1024;
pub(super) const SIGHAND: u64 = 2048;
pub(super) const PIDFD: u64 = 4096;
pub(super) const PTRACE: u64 = 8192;
pub(super) const VFORK: u64 = 16384;
pub(super) const PARENT: u64 = 32768;
pub(super) const THREAD: u64 = 65536;
pub(super) const NEWNS: u64 = 131072;
pub(super) const SYSVSEM: u64 = 262144;
pub(super) const SETTLS: u64 = 524288;
pub(super) const PARENT_SETTID: u64 = 1048576;
pub(super) const CHILD_CLEARTID: u64 = 2097152;
pub(super) const DETACHED: u64 = 4194304;
pub(super) const UNTRACED: u64 = 8388608;
pub(super) const CHILD_SETTID: u64 = 16777216;
pub(super) const NEWCGROUP: u64 = 33554432;
pub(super) const NEWUTS: u64 = 67108864;
pub(super) const NEWIPC: u64 = 134217728;
pub(super) const NEWUSER: u64 = 268435456;
pub(super) const NEWPID: u64 = 536870912;
pub(super) const NEWNET: u64 = 1073741824;
pub(super) const IO: u64 = 2147483648;
pub(super) const CLEAR_SIGHAND: u64 = 4294967296;
pub(super) const INTO_CGROUP: u64 = 8589934592;
pub(super) const NEWTIME: u64 = 128;

#[repr(C, align(8))]
#[derive(Default, Debug)]
pub(super) struct RawCloneArgs {
    pub(super) flags: u64,
    pub(super) pidfd: u64,
    pub(super) child_tid: u64,
    pub(super) parent_tid: u64,
    pub(super) exit_signal: u64,
    pub(super) stack: u64,
    pub(super) stack_size: u64,
    pub(super) tls: u64,
    pub(super) set_tid: u64,
    pub(super) set_tid_size: u64,
    pub(super) cgroup: u64,
}

pub(super) unsafe fn clone3(a0: &mut RawCloneArgs) -> i64 {
    let a1 = size_of::<RawCloneArgs>();
    let r0;
    // TODO: test with my android phone :yum:
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
}
