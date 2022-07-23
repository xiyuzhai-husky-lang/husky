use super::*;

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
            dev_src: husky_dev_utils::dev_src!(),
        }
    };
}
pub(crate) use vm_compile_error;
