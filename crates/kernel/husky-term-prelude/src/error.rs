use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum TermPreludeError {
    #[error("UniverseOverflow")]
    UniverseOverflow,
}

pub type TermPreludeResult<T> = Result<T, TermPreludeError>;
