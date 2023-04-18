use husky_expr::ExprError;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum SignatureError {
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
    DeclError,
    #[error("todo")]
    DeclExprError,
    #[error("todo")]
    ExprError,
}

impl From<&DeclError> for SignatureError {
    fn from(_: &DeclError) -> Self {
        SignatureError::DeclError
    }
}

impl From<&DeclExprError> for SignatureError {
    fn from(_: &DeclExprError) -> Self {
        SignatureError::DeclExprError
    }
}
impl From<&ExprError> for SignatureError {
    fn from(_: &ExprError) -> Self {
        SignatureError::ExprError
    }
}

impl From<&DeclarativeTermError> for SignatureError {
    fn from(value: &DeclarativeTermError) -> Self {
        SignatureError::DeclarativeTermError
    }
}

impl From<&DeclarativeTermError2> for SignatureError {
    fn from(value: &DeclarativeTermError2) -> Self {
        SignatureError::DeclarativeTermError
    }
}

impl<DB: ?Sized + DeclarativeSignatureDb> salsa::DebugWithDb<DB> for SignatureError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &DB,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

pub type DeclarativeSignatureResult<T> = Result<T, SignatureError>;
