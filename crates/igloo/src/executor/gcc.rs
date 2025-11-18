use super::{Desc, Executor};
use crate::executor::error::SelfTestErr;
use crate::util::is_executable;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Command;

pub(super) struct Gcc {
    argv0: Option<PathBuf>,
}

impl Gcc {
    fn get_version(p: PathBuf) -> Option<String> {
        // assuming `g++ (GCC) 15.2.1 20251112`
        Some(
            Command::new(p)
                .args(["--version"])
                .output()
                .ok()?
                .stdout
                .lines()
                .next()?
                .ok()?
                .split_whitespace()
                .nth(2)?
                .to_string(),
        )
    }
}

#[async_trait::async_trait]
impl Executor for Gcc {
    fn desc(&self) -> Desc {
        Desc {
            id: "gcc".to_string(),
        }
    }

    fn argv0(&self) -> Option<String> {
        self.argv0.clone().map(|p| p.to_string_lossy().into())
    }

    async fn self_test(&self) -> Result<String, SelfTestErr> {
        match self.argv0.clone() {
            Some(path) => {
                if !is_executable(path.clone()) {
                    Err(SelfTestErr::NotExecutable)
                } else {
                    Self::get_version(path).ok_or(SelfTestErr::Corrupted)
                }
            }
            None => Err(SelfTestErr::NotFound),
        }
    }
}

pub(super) fn new() -> Box<Gcc> {
    Box::new(Gcc {
        argv0: which::which("g++").ok(),
    })
}
