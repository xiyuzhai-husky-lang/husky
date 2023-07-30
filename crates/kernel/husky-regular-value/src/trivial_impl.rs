use crate::*;
use std::{
    mem::ManuallyDrop,
    panic::{RefUnwindSafe, UnwindSafe},
};

#[derive(Debug)]
pub struct __RegularValueTrivialImpl<T>(T)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __Regular for __RegularValueTrivialImpl<T>
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
{
    type __Static = __RegularValueStaticTrivialImpl<T>;
}

#[derive(Debug)]
pub struct __RegularValueStaticTrivialImpl<T>(T)
where
    T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static;

impl<T> __RegularStatic for __RegularValueStaticTrivialImpl<T>
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
    type __Static = __RegularValueStaticTrivialImpl<T>;

    unsafe fn incubate(&mut self) -> Self::__Static {
        __RegularValueStaticTrivialImpl(ManuallyDrop::take(&mut self.0))
    }
}
