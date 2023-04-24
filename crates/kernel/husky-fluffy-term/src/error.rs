use crate::*;
use husky_ethereal_signature::EtherealSignatureError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub enum FluffyTermError {
    Todo,
}

impl From<EtherealSignatureError> for FluffyTermError {
    fn from(value: EtherealSignatureError) -> Self {
        todo!()
    }
}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
pub type FluffyTermMaybeResult<T> = MaybeResult<T, FluffyTermError>;
