use thiserror::Error;

#[derive(Debug, Error)]
pub enum TermPreludeError {
    #[error("UniverseOverflow")]
    UniverseOverflow,
}

pub type TermPreludeResult<T> = Result<T, TermPreludeError>;
