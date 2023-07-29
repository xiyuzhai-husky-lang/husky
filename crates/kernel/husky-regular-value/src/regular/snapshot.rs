use super::*;

pub type __Snapshot<T> = <T as __RegularStatic>::__Snapshot;

pub trait __RegularSnapshot: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Incubator: __RegularIncubator;
}

#[derive(Debug)]
pub struct __SnapshotRefMut<T>(pub(super) Box<__Snapshot<T>>)
where
    T: __RegularStatic;

impl<T> __RegularSnapshot for __SnapshotRefMut<T>
where
    T: __RegularStatic,
{
    type __Incubator = __IncubatorRefMut<T>;
}
