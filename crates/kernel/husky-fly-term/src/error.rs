use crate::*;
use husky_dec_term::term::DecSymbolicVariableTypeErrorKind;
use husky_entity_tree::error::EntityTreeError;
use husky_eth_signature::error::EthSignatureError;
use thiserror::Error;

#[salsa::derive_debug_with_db]
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum FlyTermError {
    #[error("ethereal signature")]
    EtherealSignature(EthSignatureError),
    #[error("ethereal term")]
    EthTerm(EthTermError),
}

impl From<EthSignatureError> for FlyTermError {
    fn from(e: EthSignatureError) -> Self {
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

impl From<DecSymbolicVariableTypeErrorKind> for FlyTermError {
    fn from(value: DecSymbolicVariableTypeErrorKind) -> Self {
        todo!()
    }
}

pub type FlyTermResult<T> = Result<T, FlyTermError>;
pub type FlyTermMaybeResult<T> = MaybeResult<T, FlyTermError>;
