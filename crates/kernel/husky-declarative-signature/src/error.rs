use husky_entity_syn_tree::EntityTreeBundleError;
use husky_syn_expr::ExprError;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
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
    DeclError(DeclError),
    #[error("todo")]
    NodeDeclError,
    #[error("todo")]
    ExprError,
    #[error("todo")]
    SelfTypeNotInferred,
}

impl From<&DeclarativeSignatureError> for DeclarativeSignatureError {
    fn from(e: &DeclarativeSignatureError) -> Self {
        *e
    }
}

impl From<DeclarativeTermError2> for DeclarativeSignatureError {
    fn from(value: DeclarativeTermError2) -> Self {
        todo!()
    }
}

impl From<DeclError> for DeclarativeSignatureError {
    fn from(e: DeclError) -> Self {
        DeclarativeSignatureError::DeclError(e)
    }
}

impl From<&SynNodeDeclError> for DeclarativeSignatureError {
    fn from(_: &SynNodeDeclError) -> Self {
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
    fn from(e: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

pub type DeclarativeSignatureResult<T> = Result<T, DeclarativeSignatureError>;
