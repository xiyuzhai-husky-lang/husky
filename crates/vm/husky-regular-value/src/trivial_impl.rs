use crate::*;
use std::{
    mem::ManuallyDrop,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[derive(Debug)]
pub struct __RegularStandTrivialImpl<T>(T)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularStandTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    pub(crate) fn upcast(t: T) -> Self {
        Self(t)
    }

    pub(crate) fn downcast(self) -> T {
        self.0
    }

    pub(crate) fn downcast_ref(&self) -> &T {
        &self.0
    }

    pub(crate) fn downcast_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> __RegularStand for __RegularStandTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Snapshot = __RegularSnapshotTrivialImpl<T>;

    type __Incubator = __RegularIncubatorTrivialImpl<T>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot {
        __RegularSnapshotTrivialImpl(self.0.clone())
    }
}

#[derive(Debug)]
pub struct __RegularSnapshotTrivialImpl<T>(T)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularSnapshot for __RegularSnapshotTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Incubator = __RegularIncubatorTrivialImpl<T>;

    fn clone_into_incubator(&self) -> Self::__Incubator {
        __RegularIncubatorTrivialImpl(ManuallyDrop::new(self.0.clone()))
    }
}

#[derive(Debug)]
pub struct __RegularIncubatorTrivialImpl<T>(ManuallyDrop<T>)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularIncubator for __RegularIncubatorTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Stand = __RegularStandTrivialImpl<T>;

    unsafe fn incubate(&mut self) -> Self::__Stand {
        __RegularStandTrivialImpl(ManuallyDrop::take(&mut self.0))
    }
}
