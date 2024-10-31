use thiserror::Error;

#[derive(Debug, Error)]
pub enum VdSemExprError {
    #[error("original error({0})")]
    Original(OriginalVdSemExprError),
    #[error("derived error({0})")]
    Derived(DerivedVdSemExprError),
}

#[derive(Debug, Error)]
pub enum OriginalVdSemExprError {
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error)]
pub enum DerivedVdSemExprError {
    #[error("todo")]
    Todo,
}

pub type VdSemExprResult<T> = Result<T, VdSemExprError>;
pub type VdSemExprResultRef<'a, T> = Result<T, &'a VdSemExprError>;
