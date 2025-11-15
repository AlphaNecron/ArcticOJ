use config::Config as _Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Protocol {
    #[serde(rename = "unix")]
    Unix,
    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub protocol: Protocol,
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub listener: Address,
}

pub(super) fn load() -> Config {
    let b = _Config::builder()
        .add_source(config::File::with_name("arctic.corn"))
        .add_source(config::Environment::with_prefix("ARCTIC"));

    b.build().unwrap().try_deserialize::<Config>().unwrap()
}
