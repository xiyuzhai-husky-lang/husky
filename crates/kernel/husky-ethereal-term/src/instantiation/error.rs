use maybe_result::MaybeResult;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum EtherealTermInstantiationError {
    #[error("mismatch")]
    MisMatch,
}

pub type EtherealTermInstantiationMaybeResult<T> = MaybeResult<T, EtherealTermInstantiationError>;
