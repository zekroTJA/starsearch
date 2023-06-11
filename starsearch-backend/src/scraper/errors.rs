use thiserror::Error;

pub type Result<T, E = ScraperError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("request failed")]
    RequestError(#[from] reqwest::Error),
}
