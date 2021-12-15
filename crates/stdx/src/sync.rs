use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct ARwLock<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> ARwLock<T> {
    pub fn new(data: T) -> ARwLock<T> {
        Self {
            inner: Arc::new(RwLock::new(data)),
        }
    }
    // functional style prevents deadlock
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

impl<T> Default for ARwLock<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> Clone for ARwLock<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
