use super::sandbox::config::Config as SandboxConfig;
use config::model;
use std::sync::LazyLock;

#[model]
pub(crate) struct Config {
    #[knus(child)]
    sandbox: SandboxConfig,
}

#[model]
pub(crate) struct Worker {
    #[knus(argument)]
    id: String,

    #[knus(property)]
    cpu: u16,
}

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| config::load("igloo.kdl", "IGLOO", None).unwrap());
