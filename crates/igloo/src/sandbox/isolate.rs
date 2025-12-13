// mask user & group w an arbitrary one
#[inline]
pub(super) fn mask_ug() -> std::io::Result<()> {
    // let super::conf::ContainerConf { uid, gid, .. } = super::CONTAINER_CONF;
    // let (uid, gid) = (super::CONTAINER_CONF.uid, super::CONTAINER_CONF.gid);
    // debug!("masking uid/gid to {}:{}", uid, gid);
    // write("/proc/self/setgroups", "deny")?;

    // write("/proc/self/uid_map", format!("{0} {0} 1", GID))?;
    // write("/proc/self/gid_map", format!("{0} {0} 1", UID))?;

    // thread::set_thread_uid(Uid::from_raw(UID))?;
    // thread::set_thread_gid(Gid::from_raw(GID))?;

    Ok(())
}
