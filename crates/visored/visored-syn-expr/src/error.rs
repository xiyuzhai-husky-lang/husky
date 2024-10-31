use thiserror::Error;

#[derive(Debug, Error)]
pub enum VdSynExprError {
    #[error("original error({0})")]
    Original(OriginalVdSynExprError),
    #[error("derived error({0})")]
    Derived(DerivedVdSynExprError),
}

#[derive(Debug, Error)]
pub enum OriginalVdSynExprError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error)]
pub enum DerivedVdSynExprError {
    #[error("todo")]
    Todo,
}

pub type VdSynExprResult<T> = Result<T, VdSynExprError>;
pub type VdSynExprResultRef<'a, T> = Result<T, &'a VdSynExprError>;
