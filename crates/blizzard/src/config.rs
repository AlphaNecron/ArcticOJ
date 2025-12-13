use config::model;
use std::fmt::Debug;

const CONF_FILE: &str = "config.kdl";

#[model]
#[derive(Default)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[cfg(target_family = "unix")]
    Unix,
    #[default]
    Tcp,
}

#[model]
pub struct Address {
    #[knus(argument)]
    protocol: Protocol,
    #[knus(argument)]
    path: String,
}

#[model]
pub struct Config {
    #[knus(child, unwrap(argument))]
    database_url: String,
    #[knus(child)]
    listener: Address,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            protocol: Protocol::Tcp,
            path: "127.0.0.1:2999".to_owned(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "sqlite::memory:".to_owned(),
            listener: Address::default(),
        }
    }
}

impl Config {
    pub fn load() -> miette::Result<Self> {
        config::load(CONF_FILE, "BLIZZARD_", Some(Self::default()))
    }
}
