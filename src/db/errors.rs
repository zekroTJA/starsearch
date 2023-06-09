use thiserror::Error;

pub type Result<T, E = DatabaseError> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("failed inserting elements")]
    InsertError(#[from] meilisearch_sdk::errors::Error),
}
