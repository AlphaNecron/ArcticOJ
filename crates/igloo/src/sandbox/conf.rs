use rustix::mount::MountFlags;
use std::ffi::CString;

#[derive(knus::Decode, Debug)]
pub(super) struct ContainerConf {
    #[knus(child, unwrap(argument))]
    pub(super) hostname: String,

    #[knus(child, unwrap(argument))]
    pub(super) uid: u32,

    #[knus(child, unwrap(argument))]
    pub(super) gid: u32,

    #[knus(children(name = "bindf"))]
    pub(super) file_binds: Vec<Bind>,

    #[knus(children(name = "bind"))]
    pub(super) dir_binds: Vec<Bind>,

    #[knus(child, unwrap(argument))]
    pub(super) wd: String,

    #[knus(children(name = "mount"))]
    pub(super) mounts: Vec<Mount>,

    #[knus(children(name = "mask"), unwrap(argument))]
    pub(super) masks: Vec<String>,

    #[knus(children(name = "file"))]
    pub(super) files: Vec<File>,

    #[knus(children(name = "env"))]
    pub(super) env_vars: Vec<Env>,
}

#[derive(knus::Decode, Clone, Debug)]
pub(super) struct Bind {
    #[knus(argument)]
    pub(super) path: String,

    #[knus(property)]
    pub(super) rw: Option<bool>,

    #[knus(property)]
    pub(super) exec: Option<bool>,
}

#[derive(knus::Decode, Clone, Debug)]
pub(super) struct Mount {
    #[knus(argument)]
    pub(super) name: String,

    #[knus(argument)]
    pub(super) path: String,

    #[knus(property(name = "type"))]
    pub(super) ty: String,

    #[knus(property)]
    pub(super) opts: String,
}

#[derive(knus::Decode, Clone, Debug)]
pub(super) struct File {
    #[knus(argument)]
    pub(super) path: String,

    #[knus(argument)]
    pub(super) buf: String,

    #[knus(property)]
    pub(super) perm: u16,
}

#[derive(knus::Decode, Clone, Debug)]
pub(super) struct Env {
    #[knus(argument)]
    pub(super) key: String,

    #[knus(argument)]
    pub(super) value: String,
}

impl Bind {
    pub(super) fn flags(&self) -> MountFlags {
        let mut f =
            // MountFlags::NOEXEC |
            MountFlags::RDONLY;
        if self.rw.unwrap_or(false) {
            f ^= MountFlags::RDONLY
        }
        // if self.exec.unwrap_or(false) {
        //     f ^= MountFlags::NOEXEC
        // }
        f
    }

    pub(super) fn opts(&self) -> String {
        [
            if self.rw.unwrap_or(false) { "rw" } else { "ro" },
            // if self.exec.unwrap_or(false) {
            //     "exec"
            // } else {
            //     "noexec"
            // },
        ]
        .join(",")
    }
}
