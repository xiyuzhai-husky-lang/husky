use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_entity_syn_tree::EntityTreeBundleError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
pub enum EtherealSignatureError {
    #[error("term error")]
    TermError,
    #[error("DerivedFromDeclarative")]
    DerivedFromDeclarativeSignature(DeclarativeSignatureError),
    #[error("NoSuchItem")]
    NoSuchItem,
    #[error("EntityTreeError")]
    EntityTreeError,
}

impl From<&EtherealSignatureError> for EtherealSignatureError {
    fn from(e: &EtherealSignatureError) -> Self {
        *e
    }
}

impl From<DeclarativeSignatureError> for EtherealSignatureError {
    fn from(e: DeclarativeSignatureError) -> Self {
        EtherealSignatureError::DerivedFromDeclarativeSignature(e)
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
