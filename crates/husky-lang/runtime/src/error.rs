use std::sync::Arc;

use vm::VMError;

#[derive(Debug, Clone)]
pub struct RuntimeError {}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub type RuntimeResultArc<T> = Result<Arc<T>, RuntimeError>;

impl From<VMError> for RuntimeError {
    fn from(_: VMError) -> Self {
        todo!()
    }
}
