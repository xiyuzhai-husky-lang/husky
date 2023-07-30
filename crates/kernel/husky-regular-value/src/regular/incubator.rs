use super::*;

pub trait __RegularIncubator: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Stand: __RegularStand<__Incubator = Self>;

    unsafe fn incubate(&mut self) -> Self::__Stand;
}

/// &mut T

#[derive(Debug)]
pub struct __RegularValueIncubatorRefMut<T>(Box<(T::__Incubator, T)>)
where
    T: __RegularStand;

impl<T> __RegularIncubator for __RegularValueIncubatorRefMut<T>
where
    T: __RegularStand,
{
    type __Stand = __RegularValueStandRefMut<T>;

    unsafe fn incubate(&mut self) -> Self::__Stand {
        __RegularValueStandRefMut(&mut self.0 .1 as *mut T)
    }
}
