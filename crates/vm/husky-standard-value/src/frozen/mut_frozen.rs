use super::*;
use crate::r#static::Static;

#[derive(Debug)]
pub struct MutFrozen<T>(Box<T::Frozen>)
where
    T: Static;

impl<T> MutFrozen<T>
where
    T: Static,
{
    pub(crate) unsafe fn new(t: *mut T) -> Self {
        Self(Box::new(<T as Static>::freeze(&*t)))
    }
}

impl<T> Clone for MutFrozen<T>
where
    T: Static,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Frozen for MutFrozen<T>
where
    T: Static,
{
    type Static = *mut T;

    type Stand = Box<T::Frozen>;

    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}
