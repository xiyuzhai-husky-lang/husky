use thiserror::Error;

pub struct RefArenaPool<T, const N: usize>(Vec<T>);

impl<T, const Capacity: usize> RefArenaPool<T, Capacity> {
    pub fn new() -> Self {
        let mut inner = vec![];
        inner.reserve_exact(Capacity);
        Self(inner)
    }

    pub(super) fn alloc(&mut self, t: T) -> RefArenaPoolResult<T> {
        if self.0.len() == self.0.capacity() {
            return Err(RefArenaPoolError::RefArenaIsFull(t));
        }
        self.0.push(t);
        Ok(self.0.last().unwrap())
    }
}

pub type RefArenaPoolResult<T> = Result<*const T, RefArenaPoolError<T>>;

#[derive(Error, PartialEq, Eq, Clone)]
pub enum RefArenaPoolError<T> {
    #[error("ref arena pool is full")]
    RefArenaIsFull(T),
}
