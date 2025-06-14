use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommitGeneratorError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),

    #[error("Invalid personality: {0}")]
    InvalidPersonality(String),

    #[error("No changes detected in repository")]
    NoChanges,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
} 