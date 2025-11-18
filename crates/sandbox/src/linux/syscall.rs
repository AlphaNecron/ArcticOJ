pub(super) fn clone3(args: libc::clone_args) -> libc::c_long {
    unsafe { libc::syscall(libc::SYS_clone3, args, size_of_val(&args)) }
}
