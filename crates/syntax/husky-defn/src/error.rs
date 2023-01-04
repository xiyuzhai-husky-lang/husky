use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DefnError {
    #[error("missing body")]
    MissingBody,
}

pub type DefnResult<T> = Result<T, DefnError>;
