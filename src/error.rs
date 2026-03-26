use thiserror::Error;

#[derive(Error, Debug)]
pub enum SandboxError {
    #[error("sandbox not found: {0}")]
    NotFound(String),

    #[error("sandbox already exists: {0}")]
    AlreadyExists(String),

    #[error("sandbox is in an invalid state: {0}")]
    InvalidState(String),

    #[error("command execution failed: {0}")]
    ExecFailed(String),

    #[error("file operation failed: {0}")]
    FileError(String),

    #[error("authentication failed: {0}")]
    AuthError(String),

    #[error("provider error: {0}")]
    ProviderError(String),

    #[error("timeout after {0}s")]
    Timeout(u64),

    #[cfg(feature = "daytona")]
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SandboxError>;
