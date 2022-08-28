use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMCompileError {
    pub message: String,
    pub dev_src: DevSource,
}
pub type VMCompileResult<T> = Result<T, VMCompileError>;
