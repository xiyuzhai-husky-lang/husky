use husky_syn_expr::SynExprError;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub enum DecSignatureError {
    #[error("todo")]
    DeclarativeTermError,
    #[error("todo")]
    ParameterTypeDeclarativeTermError(u8),
    #[error("todo")]
    FieldTypeDeclarativeTermError(u8),
    #[error("todo")]
    ReturnTypeDeclarativeTermError,
    // derived
    #[error("todo")]
    DeclError(DeclError),
    #[error("todo")]
    NodeDeclError,
    #[error("todo")]
    ExprError,
    #[error("todo")]
    SelfTypeNotInferred,
}

impl From<&DecSignatureError> for DecSignatureError {
    fn from(e: &DecSignatureError) -> Self {
        *e
    }
}

impl From<DeclarativeTermError2> for DecSignatureError {
    fn from(value: DeclarativeTermError2) -> Self {
        todo!()
    }
}

impl From<DeclError> for DecSignatureError {
    fn from(e: DeclError) -> Self {
        DecSignatureError::DeclError(e)
    }
}

impl From<&SynNodeDeclError> for DecSignatureError {
    fn from(_: &SynNodeDeclError) -> Self {
        DecSignatureError::NodeDeclError
    }
}
impl From<&SynExprError> for DecSignatureError {
    fn from(_: &SynExprError) -> Self {
        DecSignatureError::ExprError
    }
}

impl From<&DeclarativeTermError> for DecSignatureError {
    fn from(value: &DeclarativeTermError) -> Self {
        DecSignatureError::DeclarativeTermError
    }
}

impl From<&DeclarativeTermError2> for DecSignatureError {
    fn from(value: &DeclarativeTermError2) -> Self {
        DecSignatureError::DeclarativeTermError
    }
}

pub type DecSignatureResult<T> = Result<T, DecSignatureError>;
