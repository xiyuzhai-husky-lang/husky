use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum ManifestError {}

pub type ManifestResult<T> = Result<T, ManifestError>;

impl From<&ManifestError> for ManifestError {
    fn from(_value: &ManifestError) -> Self {
        todo!()
    }
}
