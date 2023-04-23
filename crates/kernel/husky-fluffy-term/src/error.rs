#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FluffyTermError {}

pub type FluffyTermResult<T> = Result<T, FluffyTermError>;
