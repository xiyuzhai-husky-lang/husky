use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RuntimeError {}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub type RuntimeResultArc<T> = Result<Arc<T>, RuntimeError>;
