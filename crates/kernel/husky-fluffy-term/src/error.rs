use crate::*;
use husky_ethereal_signature::EtherealSignatureError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyTermError {}

impl From<EtherealSignatureError> for FluffyTermError {
    fn from(value: EtherealSignatureError) -> Self {
        todo!()
    }
}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
pub type FluffyTermMaybeResult<T> = MaybeResult<T, FluffyTermError>;
