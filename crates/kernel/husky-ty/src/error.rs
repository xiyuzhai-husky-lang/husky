use crate::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum TypeError {
    #[error("{0}")]
    Original(#[from] OriginalTypeError),
    #[error("{0}")]
    Derived(#[from] DerivedTypeError),
}

impl From<TermError> for TypeError {
    fn from(value: TermError) -> Self {
        todo!()
    }
}

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OriginalTypeError {
    #[error("NoSuchMethod")]
    NoSuchMethod,
    #[error("TermApplicationWrongArgumentType")]
    TermApplicationWrongArgumentType {
        parameter_ty: EtherealTerm,
        argument_ty: Either<EtherealTerm, PreludeTypePath>,
    },
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum DerivedTypeError {}
