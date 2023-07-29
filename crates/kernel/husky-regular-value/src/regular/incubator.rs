use super::*;

pub type __Incubator<T> = <T as __RegularStatic>::__Incubator;

pub trait __RegularIncubator: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Static: __RegularStatic<__Incubator = Self>;
}

/// &mut T

#[derive(Debug)]
pub struct __IncubatorRefMut<T>(Box<__Incubator<T>>)
where
    T: __RegularStatic;

impl<T> __RegularIncubator for __IncubatorRefMut<T>
where
    T: __RegularStatic,
{
    type __Static = __StaticRefMut<T>;
}
