use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub(crate) enum SelfTestErr {
    NotFound,
    NotExecutable,
    Corrupted,
}

impl Display for SelfTestErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelfTestErr::NotFound => write!(f, "couldn't find executable in PATH"),
            SelfTestErr::NotExecutable => write!(f, "program is not executable"),
            SelfTestErr::Corrupted => write!(f, "program is corrupted"),
        }
    }
}
