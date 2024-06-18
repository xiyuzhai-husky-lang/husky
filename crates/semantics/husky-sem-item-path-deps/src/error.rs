use husky_sem_expr::{SemExprDataError, SemExprTypeError};
use thiserror::Error;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Error, Clone, Copy)]
pub enum SemItemPathDepsError {
    #[error("semantic expression data error")]
    SemExprData,
    #[error("semantic expression type error")]
    SemExprType,
}

pub type SemItemPathDepsResult<T> = Result<T, SemItemPathDepsError>;

impl From<&SemExprDataError> for SemItemPathDepsError {
    fn from(value: &SemExprDataError) -> Self {
        SemItemPathDepsError::SemExprData
    }
}

impl From<&SemExprTypeError> for SemItemPathDepsError {
    fn from(value: &SemExprTypeError) -> Self {
        SemItemPathDepsError::SemExprType
    }
}

impl From<&Self> for SemItemPathDepsError {
    fn from(value: &Self) -> Self {
        *value
    }
}
