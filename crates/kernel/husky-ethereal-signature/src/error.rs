use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_entity_syn_tree::EntityTreeBundleError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum EtherealSignatureError {
    #[error("term error")]
    TermError,
    #[error("DerivedFromDeclarative")]
    DerivedFromDeclarative,
    #[error("NoSuchItem")]
    NoSuchItem,
}

impl From<&EtherealSignatureError> for EtherealSignatureError {
    fn from(e: &EtherealSignatureError) -> Self {
        *e
    }
}

impl From<DeclarativeSignatureError> for EtherealSignatureError {
    fn from(e: DeclarativeSignatureError) -> Self {
        EtherealSignatureError::DerivedFromDeclarative
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
