use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ManifestError {}

pub type ManifestResult<T> = Result<T, ManifestError>;
