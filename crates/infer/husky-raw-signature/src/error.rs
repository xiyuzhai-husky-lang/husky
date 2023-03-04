use husky_expr::ExprError;

use crate::*;

use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub enum RawSignatureError {
    RawTermError,
    ParameterTypeRawTermError(u8),
    FieldTypeRawTermError(u8),
    OutputTypeRawTermError,
    // derived
    DeclError,
    DeclExprError,
    ExprError,
}

impl From<&DeclError> for RawSignatureError {
    fn from(_: &DeclError) -> Self {
        RawSignatureError::DeclError
    }
}

impl From<&DeclExprError> for RawSignatureError {
    fn from(_: &DeclExprError) -> Self {
        RawSignatureError::DeclExprError
    }
}
impl From<&ExprError> for RawSignatureError {
    fn from(_: &ExprError) -> Self {
        RawSignatureError::ExprError
    }
}

impl<DB: ?Sized + RawSignatureDb> salsa::DebugWithDb<DB> for RawSignatureError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &DB,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

pub type RawSignatureResult<T> = Result<T, RawSignatureError>;
pub type RawSignatureResultRef<'a, T> = Result<T, &'a RawSignatureError>;
