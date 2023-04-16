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

impl From<&SignatureDeclarativeTermError> for SignatureError {
    fn from(value: &SignatureDeclarativeTermError) -> Self {
        SignatureError::DeclarativeTermError
    }
}

impl<DB: ?Sized + SignatureDb> salsa::DebugWithDb<DB> for SignatureError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &DB,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

pub type SignatureResult<T> = Result<T, SignatureError>;
