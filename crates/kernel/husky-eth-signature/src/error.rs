use crate::{
    signature::impl_block::trai_for_ty_impl_block::EthTraitForTypeImplBlockSignatureBuilderItd, *,
};
use husky_dec_signature::error::DecSignatureError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db]
pub enum EthSignatureError {
    #[error("term error")]
    TermError(EthTermError),
    #[error("DerivedFromDeclarative")]
    DerivedFromDecSignature(DecSignatureError),
    #[error("NoSuchItem")]
    NoSuchItemInTraitForTypeImplBlockEthSignatureBuilder {
        signature_builder: EthTraitForTypeImplBlockSignatureBuilderItd,
        ident: Ident,
    },
    #[error("EntityTreeError")]
    EntityTreeError,
    #[error("EntityTreeBundleError")]
    EntityTreeBundleError,
}

impl From<&EthSignatureError> for EthSignatureError {
    fn from(e: &EthSignatureError) -> Self {
        *e
    }
}

impl From<DecSignatureError> for EthSignatureError {
    fn from(e: DecSignatureError) -> Self {
        EthSignatureError::DerivedFromDecSignature(e)
    }
}

impl From<EthTermError> for EthSignatureError {
    fn from(e: EthTermError) -> Self {
        EthSignatureError::TermError(e)
    }
}

pub type EthSignatureResult<T> = Result<T, EthSignatureError>;
pub type EthSignatureMaybeResult<T> = MaybeResult<T, EthSignatureError>;
