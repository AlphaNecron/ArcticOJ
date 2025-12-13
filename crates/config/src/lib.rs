pub use config_proc_macro::model;

use miette::{IntoDiagnostic, WrapErr};

pub fn load<
    'a,
    T: std::fmt::Debug
        + serde::Deserialize<'a>
        + serde::Serialize
        + knus::DecodeChildren<knus::span::Span>,
>(
    path: &str,
    env_prefix: &str,
    fallback: Option<T>,
) -> miette::Result<T> {
    let def: T = match std::fs::read_to_string(path) {
        Err(e) => match fallback {
            Some(c) => {
                tracing::warn!(err = %e, conf = ?c, "err reading {}, falling back to defaults", path);
                c
            }
            None => Err(miette::Report::from_err(e).wrap_err(format!("err reading {}", path)))?,
        },
        Ok(buf) => knus::parse::<T>(path, &buf)?,
    };
    figment::Figment::new()
        .merge(figment::providers::Serialized::defaults(def))
        .merge(figment::providers::Env::prefixed(env_prefix))
        .extract::<T>()
        .into_diagnostic()
        .wrap_err("err parsing config")
}
