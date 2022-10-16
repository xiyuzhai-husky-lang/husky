use thiserror::Error;

#[derive(Error, Debug)]
pub enum TyInferError {
    #[error("ident unrecognized")]
    IdentUnrecognized,
}

pub type TyInferResult<T> = Result<T, TyInferError>;
