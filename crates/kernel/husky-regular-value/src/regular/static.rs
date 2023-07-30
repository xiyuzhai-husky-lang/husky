use super::*;

pub trait __RegularStatic: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Snapshot: __RegularSnapshot<__Incubator = Self::__Incubator>;
    type __Incubator: __RegularIncubator<__Static = Self>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot;
}

#[derive(Debug)]
pub struct __RegularValueStaticRefMut<T>(*mut T)
where
    T: __RegularStatic;

impl<T> __RegularStatic for __RegularValueStaticRefMut<T>
where
    T: __RegularStatic,
{
    type __Snapshot = __RegularSnapshotValueRefMut<T>;

    type __Incubator = __RegularValueIncubatorRefMut<T>;

    unsafe fn clone_into_snapshot(&self) -> Self::__Snapshot {
        __RegularSnapshotValueRefMut(Box::new((*self.0).clone_into_snapshot()))
    }
}
