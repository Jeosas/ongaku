use thiserror::Error;

#[derive(Error, Debug)]
pub enum OngakuError {
    #[error("Ongaku has already been initialized in this directory.")]
    AlreadyInitialized,
    #[error("Ongaku has not yet been initialized in this directory.")]
    NotInitialized,
    #[error("Failed to decode database.")]
    PbDecodingError(#[from] prost::DecodeError),
    #[error("Failed to decode database.")]
    PbEncodingError(#[from] prost::EncodeError),
    #[error("Failed to read/write database.")]
    DatabaseReadWriteError(#[from] std::io::Error),
    #[error("yt-dlp error: {0}")]
    YtdlpError(String),
    #[error("failed to parse json response")]
    JsonExtractError(#[from] serde_json::Error),
    #[error("unsuported url type: {0}")]
    UnsupportedUrl(String),
    #[error("{0} is already in library")]
    AlreadyInLibrary(String),
}
