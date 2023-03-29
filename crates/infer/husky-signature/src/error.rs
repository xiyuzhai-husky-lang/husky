use husky_expr::ExprError;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum SignatureError {
    #[error("todo")]
    RawTermError,
    #[error("todo")]
    ParameterTypeRawTermError(u8),
    #[error("todo")]
    FieldTypeRawTermError(u8),
    #[error("todo")]
    ReturnTypeRawTermError,
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

impl From<&SignatureRawTermError> for SignatureError {
    fn from(value: &SignatureRawTermError) -> Self {
        todo!()
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
