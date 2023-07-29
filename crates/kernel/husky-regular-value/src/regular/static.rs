use super::*;

pub trait __RegularStatic: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Snapshot: __RegularSnapshot<__Incubator = Self::__Incubator>;
    type __Incubator: __RegularIncubator<__Static = Self>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot;
}

pub type __Static<T> = <T as __Regular>::__Static;

#[derive(Debug)]
pub struct __StaticRefMut<T>(*mut T)
where
    T: __RegularStatic;

impl<T> __RegularStatic for __StaticRefMut<T>
where
    T: __RegularStatic,
{
    type __Snapshot = __SnapshotRefMut<T>;

    type __Incubator = __IncubatorRefMut<T>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot {
        __SnapshotRefMut(Box::new((*self.0).clone_into_snapshot()))
    }
}
