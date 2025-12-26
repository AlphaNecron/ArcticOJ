use crate::prelude::*;
use cryo::Req;
use rustix::cmsg_space;
use rustix::net::{RecvFlags, SendAncillaryBuffer, SendAncillaryMessage, SendFlags, recv, sendmsg};
use std::io::IoSlice;
use std::mem::MaybeUninit;
use std::os::fd::{BorrowedFd, OwnedFd};
use std::time::Duration;

#[derive(Debug)]
pub(super) struct Conn(OwnedFd);

impl Conn {
    pub(super) fn new(fd: OwnedFd) -> Self {
        Self(fd)
    }

    pub(super) fn wait(&self) -> std::io::Result<()> {
        let mut buf = [0; 1];
        // handle ready packet from `cryo`
        recv(&self.0, &mut buf, RecvFlags::empty())?;
        debug!("ack ready packet");
        // dbg!(buf);
        Ok(())
    }

    pub(super) fn send(&self, req: Req, fds: &[BorrowedFd]) -> std::io::Result<i32> {
        let mut space = [MaybeUninit::uninit(); cmsg_space!(ScmRights(3))];
        let mut anc_buf = SendAncillaryBuffer::new(&mut space);
        anc_buf.push(SendAncillaryMessage::ScmRights(fds));

        let buf = postcard::to_allocvec(&req).expect("err serializing msg");
        debug!(?req, "send");

        sendmsg(
            &self.0,
            &[IoSlice::new(&buf)],
            &mut anc_buf,
            SendFlags::empty(),
        )?;

        let mut buf: [u8; 0] = [];
        let (_, sz) = recv(&self.0, &mut buf, RecvFlags::PEEK | RecvFlags::TRUNC)?;

        let mut buf = vec![0; sz];
        recv(&self.0, &mut buf, RecvFlags::CMSG_CLOEXEC)?;

        let resp = postcard::from_bytes::<i32>(&buf).expect("err deserializing resp");
        debug!(resp = ?&resp, sz, "recv");

        Ok(resp)
    }
}
