use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VdSynExprError {
    #[error("original error({0})")]
    Original(#[from] OriginalVdSynExprError),
    #[error("derived error({0})")]
    Derived(#[from] DerivedVdSynExprError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OriginalVdSynExprError {
    #[error("empty")]
    Empty,
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DerivedVdSynExprError {
    #[error("todo")]
    Todo,
}

pub type VdSynExprResult<T> = Result<T, VdSynExprError>;
pub type VdSynExprResultRef<'a, T> = Result<T, &'a VdSynExprError>;
