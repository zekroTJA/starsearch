use thiserror::Error;

pub type Result<T, E = DatabaseError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("meilisearch error: {0}")]
    MeiliError(#[from] meilisearch_sdk::errors::Error),
}
