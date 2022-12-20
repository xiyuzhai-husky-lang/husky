use husky_absolute_path::AbsolutePathError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PackagePathError {}

pub type PackagePathResult<T> = Result<T, PackagePathError>;

impl From<&AbsolutePathError> for PackagePathError {
    fn from(_value: &AbsolutePathError) -> Self {
        todo!()
    }
}
