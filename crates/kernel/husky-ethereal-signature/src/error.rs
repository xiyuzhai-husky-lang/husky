use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_entity_tree::EntityTreeBundleError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSignatureError {
    TermError,
    DerivedFromDeclarative,
}

impl From<DeclarativeSignatureError> for EtherealSignatureError {
    fn from(e: DeclarativeSignatureError) -> Self {
        todo!()
    }
}

impl From<EtherealTermError> for EtherealSignatureError {
    fn from(e: EtherealTermError) -> Self {
        EtherealSignatureError::TermError
    }
}

impl From<&EntityTreeBundleError> for EtherealSignatureError {
    fn from(e: &EntityTreeBundleError) -> Self {
        EtherealSignatureError::TermError
    }
}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
pub type EtherealSignatureMaybeResult<T> = MaybeResult<T, EtherealSignatureError>;
