use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct __VMError(pub Arc<dyn std::any::Any>);

impl PartialEq for __VMError {
    fn eq(&self, other: &Self) -> bool {
        &*self.0 as *const dyn std::any::Any == &*other.0 as *const dyn std::any::Any
    }
}
impl Eq for __VMError {}

impl std::fmt::Display for __VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub type __VMResult<T> = Result<T, __VMError>;
