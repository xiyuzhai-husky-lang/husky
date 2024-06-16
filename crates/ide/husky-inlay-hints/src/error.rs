use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum InlayHintError {}

pub type InlayHintResult<T> = Result<T, InlayHintError>;
