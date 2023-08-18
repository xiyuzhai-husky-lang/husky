use crate::*;
use husky_declarative_signature::DeclarativeSignatureError;
use husky_entity_syn_tree::EntityTreeBundleError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EtherealSignatureDb)]
pub enum EtherealSignatureError {
    #[error("term error")]
    TermError(EtherealTermError),
    #[error("DerivedFromDeclarative")]
    DerivedFromDeclarativeSignature(DeclarativeSignatureError),
    #[error("NoSuchItem")]
    NoSuchItemInTraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated {
        template_partially_instantiated:
            TraitForTypeImplBlockEtherealSignatureTemplatePartiallyInstantiated,
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
    fn from(e: &EntityTreeBundleError) -> Self {
        EtherealSignatureError::EntityTreeBundleError
    }
}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
pub type EtherealSignatureMaybeResult<T> = MaybeResult<T, EtherealSignatureError>;
