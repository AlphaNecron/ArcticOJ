use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const CONF_FILE: &str = "config.kdl";

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Default, config::Scalar)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[cfg(target_family = "unix")]
    Unix,
    #[default]
    Tcp,
}

#[derive(Debug, Deserialize, Serialize, config::Object)]
pub struct Address {
    #[knus(argument)]
    pub protocol: Protocol,
    #[knus(argument)]
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize, config::Object)]
pub struct Config {
    #[knus(child, unwrap(argument))]
    pub database_url: String,
    #[knus(child)]
    pub listener: Address,
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
