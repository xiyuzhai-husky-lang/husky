use super::*;
use crate::snapshot::{Snapshot, SnapshotDyn};

/// Stand is the static version of a type
pub trait Static: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type Snapshot: Snapshot<Static = Self>;
    unsafe fn snapshot(&self) -> Self::Snapshot;
}

#[derive(Debug)]
pub struct RefMutSnapshot<T>(Box<T::Snapshot>)
where
    T: Static;

impl<T> Clone for RefMutSnapshot<T>
where
    T: Static,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Snapshot for RefMutSnapshot<T>
where
    T: Static,
{
    type Static = *mut T;

    type Stand = Box<T::Snapshot>;

    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}

impl<T> Static for *mut T
where
    T: Static,
{
    type Snapshot = RefMutSnapshot<T>;

    unsafe fn snapshot(&self) -> Self::Snapshot {
        RefMutSnapshot(Box::new((**self).snapshot()))
    }
}

pub trait StaticDyn: std::fmt::Debug + std::any::Any + RefUnwindSafe + UnwindSafe {
    unsafe fn snapshot(&self) -> Arc<dyn SnapshotDyn>;
}

impl<T> StaticDyn for T
where
    T: Static,
{
    unsafe fn snapshot(&self) -> Arc<dyn SnapshotDyn> {
        Arc::new(self.snapshot())
    }
}
