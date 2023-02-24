use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DefnError {
    #[error("expect body")]
    ExpectBody,
}

pub type DefnResult<T> = Result<T, DefnError>;
