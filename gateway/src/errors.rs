use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse integer: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Serialization error: {0}")]
    SerialError(#[from] serde_json::Error),

    #[error("MongoDB Error: {0}")]
    DBError(#[from] oximod::_mongodb::error::Error),

    #[error("Oximod Error: {0}")]
    OximodError(#[from] oximod_core::error::oximod_error::OxiModError),

    #[error("Std Env var Error: {0}")]
    StdVarError(#[from] std::env::VarError),

    #[error("Poem Error: {0}")]
    PoemError(#[from] poem::error::Error),
}

/// Result type.
pub type Result<T> = std::result::Result<T, Error>;
