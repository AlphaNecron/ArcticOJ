use cryo::*;
use rustix::cmsg_space;
use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendFlags, recv, recvmsg, send,
};
use rustix::stdio::take_stdin;
use std::io::IoSliceMut;
use std::mem::MaybeUninit;
use std::time::Duration;

macro_rules! sendobj {
    ($sock: expr, $obj:expr) => {{
        let vec = postcard::to_allocvec($obj).expect("err serializing msg");
        send($sock, &vec, SendFlags::empty())
    }};
}

fn main() -> std::io::Result<()> {
    let sock = unsafe { take_stdin() };
    // send a packet to `igloo`
    send(&sock, &[0], SendFlags::empty())?;
    loop {
        eprintln!("loop entered");
        let mut buf: [u8; 0] = [];
        let (_, sz) = recv(&sock, &mut buf, RecvFlags::PEEK | RecvFlags::TRUNC)?;

        dbg!(sz);

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

        let m = recvmsg(
            &sock,
            &mut [IoSliceMut::new(&mut buf)],
            &mut anc_buf,
            RecvFlags::empty(),
        )?;

        let fds = anc_buf
            .drain()
            .map(|a| match a {
                RecvAncillaryMessage::ScmRights(fds) => fds.collect(),
                _ => vec![],
            })
            .flatten()
            .collect::<Vec<_>>();

        eprintln!("fd {}", fds.len());

        // TODO: either skip this msg or crash
        match postcard::from_bytes::<Req>(&buf).expect("err parsing msg") {
            Req::Compile(_) => {
                eprintln!("compile")
            }
            Req::Exec(_) => {
                eprintln!("exec")
            }
        }

        let resp = Resp::Compile(RUsage {
            cpu_time: Duration::new(0, 0),
            mem: 456,
        });

        sendobj!(&sock, &resp)?;
    }
}
