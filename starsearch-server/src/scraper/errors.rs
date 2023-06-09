use thiserror::Error;

use crate::db::errors::DatabaseError;

pub type Result<T, E = ScraperError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
}
