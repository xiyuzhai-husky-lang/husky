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
    NodeDeclError,
    #[error("todo")]
    ExprError,
}

impl From<DeclError> for DeclarativeSignatureError {
    fn from(_: DeclError) -> Self {
        DeclarativeSignatureError::DeclError
    }
}

impl From<&NodeDeclError> for DeclarativeSignatureError {
    fn from(_: &NodeDeclError) -> Self {
        DeclarativeSignatureError::NodeDeclError
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

pub type DeclarativeSignatureResult<T> = Result<T, DeclarativeSignatureError>;
