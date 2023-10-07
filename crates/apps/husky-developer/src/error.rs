use crate::*;

#[derive(Debug, thiserror::Error)]
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
