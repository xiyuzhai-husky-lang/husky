use husky_syn_expr::SynExprError;
use thiserror::Error;

use crate::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum DecSignatureError {
    #[error("todo")]
    DecTermError,
    #[error("todo")]
    ParameterTypeDecTermError(u8),
    #[error("todo")]
    FieldTypeDecTermError(u8),
    #[error("todo")]
    ReturnTypeDecTermError,
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

impl From<DecTermError2> for DecSignatureError {
    fn from(value: DecTermError2) -> Self {
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

impl From<&DecTermError> for DecSignatureError {
    fn from(value: &DecTermError) -> Self {
        DecSignatureError::DecTermError
    }
}

impl From<&DecTermError2> for DecSignatureError {
    fn from(value: &DecTermError2) -> Self {
        DecSignatureError::DecTermError
    }
}

pub type DecSignatureResult<T> = Result<T, DecSignatureError>;
