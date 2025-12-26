use cryo::*;
use rustix::cmsg_space;
use rustix::fs::{Mode, chmod};
use rustix::mount::{MountFlags, mount, mount_bind, mount_remount};
use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendFlags, recv, recvmsg, send,
};
use rustix::process::{Resource, Rlimit, WaitOptions, setrlimit, waitpid};
use rustix::runtime::{execve, exit_group};
use rustix::stdio::{dup2_stderr, dup2_stdin, dup2_stdout, take_stdin};
use rustix::thread::LinkNameSpaceType::Mount;
use rustix::thread::{Gid, Pid, Uid, set_no_new_privs, set_thread_gid, set_thread_uid};
use std::env;
use std::env::args;
use std::ffi::CString;
use std::fs::write;
use std::io::{Error, IoSliceMut};
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, OwnedFd};

macro_rules! sendobj {
    ($sock: expr, $obj:expr) => {{
        let vec = postcard::to_allocvec($obj).expect("err serializing msg");
        send($sock, &vec, SendFlags::empty())
    }};
}

macro_rules! setrlm {
    ($res: ident, $soft: expr, $hard: expr) => {
        setrlimit(
            Resource::$res,
            Rlimit {
                current: Some($soft as u64),
                maximum: Some($hard as u64),
            },
        )
    };
}

macro_rules! bindfd {
    ($fd: expr, $wd: expr, $fname: expr, $mflags: expr, $fmod: expr) => {{
        let mnt = format!("{}/{}", $wd, $fname);
        write(&mnt, &[])?;
        mount_bind(format!("/proc/self/fd/{}", &$fd.as_raw_fd()), &mnt)?;
        mount_remount(&mnt, $mflags, "")?;
        chmod(&mnt, $fmod)
    }};
}

// TODO: impl seccomp
// TODO: impl landlock
// TODO: impl caps clear
#[inline]
fn spawn(
    rlm: &RLimit,
    cgfd: &OwnedFd,
    args: &Vec<String>,
    child: impl FnOnce() -> std::io::Result<()>,
    par: impl FnOnce(u32) -> std::io::Result<()>,
) -> std::io::Result<()> {
    let mut cl_args = clone3::RawCloneArgs {
        flags: clone3::INTO_CGROUP,
        cgroup: cgfd.as_raw_fd() as u64,
        // SIGCHLD
        exit_signal: 17,
        ..Default::default()
    };

    let tl = rlm.time.as_secs_f32();

    match clone3::clone3(&mut cl_args)? {
        clone3::Fork::Child => {
            let e = {
                let args = args
                    .iter()
                    .map(|s| CString::new(s.as_str()))
                    .collect::<Result<Vec<_>, _>>()?;

                let mut cargs: Vec<_> = args
                    .iter()
                    .map(|s| s.as_bytes_with_nul().as_ptr())
                    .collect();

                cargs.push(std::ptr::null());
                let cenv = [std::ptr::null()];

                setrlm!(Cpu, tl.floor(), tl.ceil());
                setrlm!(Nproc, 1, 1);
                setrlm!(Core, 0, 0);
                setrlm!(Fsize, rlm.output, rlm.output);
                setrlm!(As, rlm.mem, rlm.mem);

                set_thread_uid(Uid::from_raw(UID))?;
                set_thread_gid(Gid::from_raw(GID))?;
                set_no_new_privs(true)?;

                child()?;

                unsafe { execve(&args[0], cargs.as_ptr(), cenv.as_ptr()) }
            };

            eprintln!("setup/execve failed (e={e:?})");

            exit_group(1);
        }
        clone3::Fork::Parent(pid) => par(pid)?,
    };
    Ok(())
}

fn main() -> std::io::Result<()> {
    let sock = unsafe { take_stdin() };
    // send a packet to `igloo`
    send(&sock, &[0], SendFlags::empty())?;

    // why not env? cuz im too lazy to mutate container conf :c
    let wd = args().nth(1).unwrap();

    loop {
        // eprintln!("loop entered");
        let mut buf: [u8; 0] = [];
        let (_, sz) = recv(&sock, &mut buf, RecvFlags::PEEK | RecvFlags::TRUNC)?;

        if sz == 0 {
            break;
        }

        // dbg!(sz);

        let mut buf = vec![0; sz];
        // - compile
        // + 1 for cgroup fd
        // + 1 for compiler output
        // + 1 for src file
        // - exec
        // + 1 for cgroup fd
        // + 1 for inp
        // + 1 for out
        let mut anc_space = [MaybeUninit::uninit(); cmsg_space!(ScmRights(3))];
        let mut anc_buf = RecvAncillaryBuffer::new(&mut anc_space);

        recvmsg(
            &sock,
            &mut [IoSliceMut::new(&mut buf)],
            &mut anc_buf,
            RecvFlags::empty(),
        )?;

        let fds: Vec<_> = anc_buf
            .drain()
            .flat_map(|a| match a {
                RecvAncillaryMessage::ScmRights(fds) => fds.collect(),
                _ => vec![],
            })
            .collect();

        // eprintln!("fd {}", fds.len());

        // TODO: either skip this msg or crash
        match postcard::from_bytes::<Req>(&buf).expect("err parsing msg") {
            // here expects m, n exists where args[m] = {{input}} and args[n] = {{output}} (for templating ykyk)
            Req::Compile(rlm, args) => {
                spawn(
                    &rlm,
                    &fds[0],
                    &args,
                    || {
                        // forward std{out,err} to compiler output fd
                        dup2_stdout(&fds[1])?;
                        dup2_stderr(&fds[1])?;

                        Ok(())
                    },
                    |pid| {
                        let (_, ws) = waitpid(Pid::from_raw(pid as i32), WaitOptions::empty())
                            .unwrap()
                            .unwrap();

                        // cpu_time and mem_usage can be collected from igloo so Ill just return the exit code here
                        sendobj!(&sock, &ws.exit_status().unwrap())
                            .and(Ok(()))
                            .map_err(|x| x.into())
                    },
                )?;
            }
            Req::Exec(rlm, io, args) => spawn(
                &rlm,
                &fds[0],
                &args,
                || {
                    if let Some(inp) = io.f_inp {
                        bindfd!(
                            fds[1],
                            wd,
                            inp,
                            MountFlags::NOEXEC | MountFlags::RDONLY,
                            Mode::RUSR
                        )?;
                    } else {
                        dup2_stdin(&fds[1])?;
                    }

                    if let Some(out) = io.f_out {
                        bindfd!(fds[2], wd, out, MountFlags::NOEXEC, Mode::WUSR)?;
                    } else {
                        dup2_stdout(&fds[2])?;
                    }

                    Ok(())
                },
                |_| Ok(()),
            )?,
        }
    }
    Ok(())
}
