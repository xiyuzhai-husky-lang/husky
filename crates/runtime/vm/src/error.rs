use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMRuntimeError {
    pub message: String,
}
pub type VMRuntimeResult<T> = Result<T, VMRuntimeError>;
#[macro_export]
macro_rules! vm_runtime_error {
    ($message: expr) => {
        VMRuntimeError {
            message: $message.into(),
        }
    };
}
use dev_utils::DevSource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMCompileError {
    pub message: String,
    pub dev_src: DevSource,
}
pub type VMCompileResult<T> = Result<T, VMCompileError>;
macro_rules! vm_compile_error {
    ($message: expr) => {
        VMCompileError {
            message: $message.into(),
            dev_src: dev_utils::dev_src!(),
        }
    };
}
pub(crate) use vm_compile_error;
