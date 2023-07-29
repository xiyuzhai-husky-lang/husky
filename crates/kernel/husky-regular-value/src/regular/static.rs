use super::*;

pub trait __RegularStatic: std::fmt::Debug + RefUnwindSafe + UnwindSafe {
    type __Snapshot: __RegularSnapshot<__Stand = Self::__Stand>;
    type __Stand: __RegularStand<__Static = Self>;
}

pub type __Static<T> = <T as __Regular>::__Static;

#[derive(Debug)]
pub struct __StaticRefMut<T>(*mut __Static<T>)
where
    T: __Regular;

impl<T> __RegularStatic for __StaticRefMut<T>
where
    T: __Regular,
{
    type __Snapshot = __SnapshotRefMut<T>;

    type __Stand = __StandRefMut<T>;
}
