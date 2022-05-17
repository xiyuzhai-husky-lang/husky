use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMRuntimeError {
    pub message: String,
}
pub type VMRuntimeResult<T> = Result<T, VMRuntimeError>;
macro_rules! vm_runtime_error {
    ($message: expr) => {
        VMRuntimeError {
            message: $message.into(),
        }
    };
}
pub(crate) use vm_runtime_error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMCompileError {
    pub message: String,
}
pub type VMCompileResult<T> = Result<T, VMCompileError>;
macro_rules! vm_compile_error {
    ($message: expr) => {
        VMCompileError {
            message: $message.into(),
        }
    };
}
pub(crate) use vm_compile_error;
