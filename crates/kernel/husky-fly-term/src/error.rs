use crate::*;
use husky_dec_term::term::DecTermSymbolTypeErrorKind;
use husky_entity_tree::EntityTreeError;
use husky_eth_signature::EtherealSignatureError;
use thiserror::Error;

#[salsa::debug_with_db]
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum FlyTermError {
    #[error("ethereal signature")]
    EtherealSignature(EtherealSignatureError),
    #[error("ethereal term")]
    EthTerm(EthTermError),
}

impl From<EtherealSignatureError> for FlyTermError {
    fn from(e: EtherealSignatureError) -> Self {
        FlyTermError::EtherealSignature(e)
    }
}

impl From<EthTermError> for FlyTermError {
    fn from(e: EthTermError) -> Self {
        FlyTermError::EthTerm(e)
    }
}

impl From<&EntityTreeError> for FlyTermError {
    fn from(value: &EntityTreeError) -> Self {
        todo!()
    }
}

impl From<DecTermSymbolTypeErrorKind> for FlyTermError {
    fn from(value: DecTermSymbolTypeErrorKind) -> Self {
        todo!()
    }
}

pub type FlyTermResult<T> = Result<T, FlyTermError>;
pub type FlyTermMaybeResult<T> = MaybeResult<T, FlyTermError>;