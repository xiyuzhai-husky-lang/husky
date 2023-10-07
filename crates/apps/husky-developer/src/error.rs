use crate::*;

#[derive(Debug, Error)]
pub enum DeveloperError {}

pub type DeveloperResult<T> = Result<T, DeveloperError>;

impl From<&'static str> for DeveloperError {
    fn from(_: &'static str) -> Self {
        todo!()
    }
}

impl From<std::io::Error> for DeveloperError {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

impl std::fmt::Display for DeveloperError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
