use config::model;
use rustix::mount::MountFlags;

#[model]
pub(crate) struct Config {
    #[knus(child, unwrap(argument))]
    domain_name: String,

    #[knus(child, unwrap(argument))]
    uid: u32,

    #[knus(child, unwrap(argument))]
    gid: u32,

    #[knus(children(name = "env"))]
    env_vars: Vec<Env>,

    #[knus(child)]
    fs: FS,
}

#[model]
pub(crate) struct FS {
    #[knus(argument)]
    root: String,

    #[knus(children(name = "bindf"))]
    file_binds: Vec<Bind>,

    #[knus(children(name = "bind"))]
    dir_binds: Vec<Bind>,

    #[knus(child, unwrap(argument))]
    wd: String,

    #[knus(children(name = "mount"))]
    mounts: Vec<Mount>,

    #[knus(children(name = "mask"), unwrap(argument))]
    masks: Vec<String>,

    #[knus(children(name = "file"))]
    files: Vec<File>,
}

#[model]
pub(crate) struct Bind {
    #[knus(argument)]
    path: String,

    #[knus(property)]
    rw: Option<bool>,

    #[knus(property)]
    exec: Option<bool>,
}

#[model]
pub(crate) struct Mount {
    #[knus(argument)]
    name: String,

    #[knus(argument)]
    path: String,

    #[knus(property(name = "type"))]
    ty: String,

    #[knus(property)]
    opts: String,
}

#[model]
pub(crate) struct File {
    #[knus(argument)]
    path: String,

    #[knus(argument)]
    buf: String,

    #[knus(property)]
    perm: u16,
}

#[model]
pub(crate) struct Env {
    #[knus(argument)]
    key: String,

    #[knus(argument)]
    value: String,
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
