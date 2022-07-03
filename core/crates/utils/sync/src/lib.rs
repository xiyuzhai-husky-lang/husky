use std::sync::{Arc, RwLock};

pub type ASafeRwLock<T> = Arc<SafeRwLock<T>>;

#[derive(Debug)]
pub struct SafeRwLock<T> {
    inner: RwLock<T>,
}

impl<T> SafeRwLock<T> {
    pub fn new(data: T) -> SafeRwLock<T> {
        Self {
            inner: RwLock::new(data),
        }
    }
    // declarative style prevents deadlock
    pub fn read<F, S>(&self, f: F) -> S
    where
        F: FnOnce(&T) -> S,
    {
        return f(&self.inner.read().unwrap());
    }
    pub fn write<F, S>(&self, f: F) -> S
    where
        F: FnOnce(&mut T) -> S,
    {
        return f(&mut self.inner.write().unwrap());
    }

    pub fn clone_to_arc(&self) -> Arc<T>
    where
        T: Clone,
    {
        self.read(|t| Arc::new(t.clone()))
    }
}

impl<T> Default for SafeRwLock<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}
