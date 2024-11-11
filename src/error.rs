use thiserror::Error;

#[derive(Error, Debug)]
pub enum OngakuError {
    #[error("Ongaku has already been initialized in this directory.")]
    AlreadyInitialized,
    #[error("Ongaku has not yet been initialized in this directory.")]
    NotInitialized,
    #[error("Failed to right database: {0}")]
    DbWriteFailed(String),
}
