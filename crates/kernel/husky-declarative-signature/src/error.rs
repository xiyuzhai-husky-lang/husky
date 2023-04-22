use husky_entity_tree::EntityTreeBundleError;
use husky_expr::ExprError;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum DeclarativeSignatureError {
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

impl From<&DeclError> for DeclarativeSignatureError {
    fn from(_: &DeclError) -> Self {
        DeclarativeSignatureError::DeclError
    }
}

impl From<&DeclExprError> for DeclarativeSignatureError {
    fn from(_: &DeclExprError) -> Self {
        DeclarativeSignatureError::DeclExprError
    }
}
impl From<&ExprError> for DeclarativeSignatureError {
    fn from(_: &ExprError) -> Self {
        DeclarativeSignatureError::ExprError
    }
}

impl From<&DeclarativeTermError> for DeclarativeSignatureError {
    fn from(value: &DeclarativeTermError) -> Self {
        DeclarativeSignatureError::DeclarativeTermError
    }
}

impl From<&DeclarativeTermError2> for DeclarativeSignatureError {
    fn from(value: &DeclarativeTermError2) -> Self {
        DeclarativeSignatureError::DeclarativeTermError
    }
}

impl From<&EntityTreeBundleError> for DeclarativeSignatureError {
    fn from(value: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl<DB: ?Sized + DeclarativeSignatureDb> salsa::DebugWithDb<DB> for DeclarativeSignatureError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &DB,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

pub type DeclarativeSignatureResult<T> = Result<T, DeclarativeSignatureError>;
