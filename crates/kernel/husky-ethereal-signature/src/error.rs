use crate::*;
use husky_declarative_signature::DecSignatureError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db]
pub enum EtherealSignatureError {
    #[error("term error")]
    TermError(EthTermError),
    #[error("DerivedFromDeclarative")]
    DerivedFromDecSignature(DecSignatureError),
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

impl From<DecSignatureError> for EtherealSignatureError {
    fn from(e: DecSignatureError) -> Self {
        EtherealSignatureError::DerivedFromDecSignature(e)
    }
}

impl From<EthTermError> for EtherealSignatureError {
    fn from(e: EthTermError) -> Self {
        EtherealSignatureError::TermError(e)
    }
}

pub type EtherealSignatureResult<T> = Result<T, EtherealSignatureError>;
pub type EtherealSignatureMaybeResult<T> = MaybeResult<T, EtherealSignatureError>;
