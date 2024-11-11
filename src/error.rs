use thiserror::Error;

#[derive(Error, Debug)]
pub enum OngakuError {
    #[error("Ongaku has already been initialized in this directory.")]
    AlreadyInitialized,
    #[error("Ongaku has not yet been initialized in this directory.")]
    NotInitialized,
    #[error(transparent)]
    PbDecodingError(#[from] prost::DecodeError),
    #[error(transparent)]
    PbEncodingError(#[from] prost::EncodeError),
    #[error(transparent)]
    DatabaseReadWriteError(#[from] std::io::Error),
}
