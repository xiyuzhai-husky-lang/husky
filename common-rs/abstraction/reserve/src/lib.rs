use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

pub struct Reserve<T>
where
    T: PartialEq + Eq + Clone,
{
    data: T,
    drained: Arc<Mutex<bool>>,
}

impl<T> PartialEq for Reserve<T>
where
    T: PartialEq + Eq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Clone for Reserve<T>
where
    T: PartialEq + Eq + Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            drained: self.drained.clone(),
        }
    }
}

impl<T> Eq for Reserve<T> where T: PartialEq + Eq + Clone {}

impl<T> std::fmt::Debug for Reserve<T>
where
    T: PartialEq + Eq + std::fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reserve")
            .field("data", &self.data)
            .field("drained", &*self.drained.lock().unwrap())
            .finish()
    }
}

impl<T> Reserve<T>
where
    T: PartialEq + Eq + Clone,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            drained: Arc::new(Mutex::new(false)),
        }
    }
    pub fn release(&self, f: impl FnOnce(&T)) {
        let mut drained = self.drained.lock().unwrap();
        if !*drained {
            *drained = true;
            f(&self.data);
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> Deref for Reserve<T>
where
    T: PartialEq + Eq + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
