use super::{RtDesc, Runtime};
use crate::runtime::error::SelfTestErr;
use crate::util;
use regex::Regex;
use std::process::Command;

#[derive(Clone)]
enum Type {
    PyPy2,
    PyPy3,
    Py2,
    Py3,
}

pub(super) struct Python {
    ty: Type,
    argv0: Option<String>,
}

impl Python {
    fn get_version(argv0: String, ty: Type) -> Option<String> {
        let py_pattern = Regex::new(r"^Python (?<py>[0-9]\.[0-9]+\.[0-9]+)").unwrap();
        let pypy_pattern = Regex::new(
            r"^\[PyPy (?<pypy>[0-9]\.[0-9]+\.[0-9]+) with GCC (?<gcc>[0-9]+\.[0-9]+\.[0-9]+)",
        )
        .unwrap();

        let o = Command::new(argv0)
            .args(["--version"])
            .output()
            .map(|o| {
                String::from_utf8(o.stdout)
                    .map(|x| x.lines().map(|x| x.to_string()).collect::<Vec<String>>())
                    .ok()
            })
            .ok()
            .flatten()?;

        if match ty {
            Type::Py3 => o.is_empty(),
            Type::PyPy3 => o.len() < 2,
            _ => false,
        } {
            return None;
        }

        let py = py_pattern.captures(&o[0]).map(|m| m["py"].to_string())?;

        match ty {
            Type::Py3 => Some(py),
            Type::PyPy3 => pypy_pattern
                .captures(&o[1])
                .map(|m| format!("{} with Python {}", &m["pypy"], py)),
            _ => None,
        }
    }
}

#[async_trait::async_trait]
impl Runtime for Python {
    fn desc(&self) -> RtDesc {
        RtDesc {
            id: match self.ty.clone() {
                Type::PyPy2 => "pypy2",
                Type::PyPy3 => "pypy3",
                Type::Py2 => "python2",
                Type::Py3 => "python3",
            }
            .to_string(),
        }
    }

    fn argv0(&self) -> Option<String> {
        self.argv0.clone()
    }

    async fn self_test(&self) -> Result<String, SelfTestErr> {
        match self.argv0.clone() {
            Some(path) => {
                if !util::is_executable(path.clone()) {
                    Err(SelfTestErr::NotExecutable)
                } else {
                    Self::get_version(path.clone(), self.ty.clone()).ok_or(SelfTestErr::Corrupted)
                }
            }
            None => Err(SelfTestErr::NotFound),
        }
    }
}

fn new(ty: Type, exec: &str) -> Box<Python> {
    Box::new(Python {
        ty,
        argv0: util::which(exec).map(|x| x.to_string_lossy().into()),
    })
}

pub(super) fn python3() -> Box<Python> {
    new(Type::Py3, "python3")
}

pub(super) fn pypy3() -> Box<Python> {
    new(Type::PyPy3, "pypy3")
}
