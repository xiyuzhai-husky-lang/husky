use crate::*;
use std::{
    mem::ManuallyDrop,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[derive(Debug)]
pub struct __RegularValueStandTrivialImpl<T>(T)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularValueStandTrivialImpl<T>
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

impl<T> __RegularStand for __RegularValueStandTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Snapshot = __RegularValueSnapshotTrivialImpl<T>;

    type __Incubator = __RegularValueIncubatorTrivialImpl<T>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot {
        __RegularValueSnapshotTrivialImpl(self.0.clone())
    }
}

#[derive(Debug)]
pub struct __RegularValueSnapshotTrivialImpl<T>(T)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularSnapshot for __RegularValueSnapshotTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Incubator = __RegularValueIncubatorTrivialImpl<T>;

    fn clone_into_incubator_box(&self) -> Box<dyn __RegularIncubatorDyn> {
        Box::new(__RegularValueIncubatorTrivialImpl(ManuallyDrop::new(
            self.0.clone(),
        )))
    }
}

#[derive(Debug)]
pub struct __RegularValueIncubatorTrivialImpl<T>(ManuallyDrop<T>)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularIncubator for __RegularValueIncubatorTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Stand = __RegularValueStandTrivialImpl<T>;

    unsafe fn incubate(&mut self) -> Self::__Stand {
        __RegularValueStandTrivialImpl(ManuallyDrop::take(&mut self.0))
    }
}
