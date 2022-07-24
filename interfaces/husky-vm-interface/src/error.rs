use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct __VMError(pub Arc<dyn std::any::Any>);

impl std::fmt::Display for __VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type __VMResult<T> = Result<T, __VMError>;
