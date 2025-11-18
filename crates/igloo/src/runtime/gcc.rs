use super::{RtDesc, Runtime};
use crate::runtime::error::SelfTestErr;
use crate::util::is_executable;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Command;

pub(super) struct GCC {
    argv0: Option<PathBuf>,
}

impl GCC {
    fn get_version(p: PathBuf) -> Option<String> {
        match Command::new(p).args(["--version"]).output() {
            Ok(out) => {
                match out.stdout.lines().next() {
                    Some(line) => {
                        // assuming `g++ (GCC) 15.2.1 20251112`
                        line.map(|l| l.split_whitespace().nth(2).unwrap().to_string())
                            .ok()
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[async_trait::async_trait]
impl Runtime for GCC {
    fn desc(&self) -> RtDesc {
        RtDesc {
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

pub(super) fn new() -> Box<GCC> {
    Box::new(GCC {
        argv0: which::which("g++").ok(),
    })
}
