use thiserror::Error;

#[derive(Error, Debug)]
pub enum InferError {
    #[error("ident unrecognized")]
    IdentUnrecognized,
    #[error("derived")]
    Derived,
}

pub type InferResult<T> = Result<T, InferError>;
