use std::num::ParseIntError;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bad url")]
    BadUrl(#[from] url::ParseError),
    #[error("Bad error")]
    ConsulError,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Session flag is required to acquire lock")]
    RequireSessionFlag,
    #[error("Error parsing X-Consul-Index")]
    ParseConsulIndexError(#[from] ParseIntError),
}
