use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use common::Formatter;

pub struct Reserve<T>
where
    T: PartialEq + Eq,
{
    data: T,
    drained: Arc<Mutex<bool>>,
}

impl<T> PartialEq for Reserve<T>
where
    T: PartialEq + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Eq for Reserve<T> where T: PartialEq + Eq {}

impl<T> std::fmt::Debug for Reserve<T>
where
    T: PartialEq + Eq + std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reserve")
            .field("data", &self.data)
            .field("drained", &*self.drained.lock().unwrap())
            .finish()
    }
}

impl<T> Reserve<T>
where
    T: PartialEq + Eq,
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
}

impl<T> Deref for Reserve<T>
where
    T: PartialEq + Eq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
