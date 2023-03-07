use husky_expr::ExprError;

use crate::*;

use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SignatureError {
    RawTermError,
    ParameterTypeRawTermError(u8),
    FieldTypeRawTermError(u8),
    OutputTypeRawTermError,
    // derived
    DeclError,
    DeclExprError,
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

impl<DB: ?Sized + SignatureDb> salsa::DebugWithDb<DB> for SignatureError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &DB,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

pub type SignatureResult<T> = Result<T, SignatureError>;
