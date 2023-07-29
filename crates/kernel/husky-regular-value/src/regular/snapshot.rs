use super::*;

pub type __Snapshot<T> = <<T as __Regular>::__Static as __RegularStatic>::__Snapshot;

pub trait __RegularSnapshot: std::fmt::Debug + RefUnwindSafe + UnwindSafe {
    type __Stand: __RegularStand;
}

#[derive(Debug)]
pub struct __SnapshotRefMut<T>(Box<__Snapshot<T>>)
where
    T: __Regular;

impl<T> __RegularSnapshot for __SnapshotRefMut<T>
where
    T: __Regular,
{
    type __Stand = __StandRefMut<T>;
}
