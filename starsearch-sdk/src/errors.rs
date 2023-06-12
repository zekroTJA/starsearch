use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("request failed: {0}")]
    RequestError(#[from] reqwest::Error),
}
