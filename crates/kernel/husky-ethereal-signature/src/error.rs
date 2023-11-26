use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_entity_syn_tree::EntityTreeBundleError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub enum EtherealSignatureError {
    #[error("term error")]
    TermError(EtherealTermError),
    #[error("DerivedFromDeclarative")]
    DerivedFromDeclarativeSignature(DeclarativeSignatureError),
    #[error("NoSuchItem")]
    NoSuchItemInTraitForTypeImplBlockEtherealSignatureBuilder {
        signature_builder: TraitForTypeImplBlockEtherealSignatureBuilder,
        ident: Ident,
    },
    #[error("EntityTreeError")]
    EntityTreeError,
    #[error("EntityTreeBundleError")]
    EntityTreeBundleError,
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
        EtherealSignatureError::TermError(e)
    }
}

impl From<&EntityTreeBundleError> for EtherealSignatureError {
    fn from(_e: &EntityTreeBundleError) -> Self {
        EtherealSignatureError::EntityTreeBundleError
    }
}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
pub type EtherealSignatureMaybeResult<T> = MaybeResult<T, EtherealSignatureError>;
