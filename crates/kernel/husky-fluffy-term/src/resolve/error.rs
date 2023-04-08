use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FluffyTermResolveError {}

pub type FluffyTermResolveResult<T> = Result<T, FluffyTermResolveError>;
