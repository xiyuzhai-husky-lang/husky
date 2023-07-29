use super::*;

pub type __Stand<T> = <<T as __Regular>::__Static as __RegularStatic>::__Stand;

pub trait __RegularStand: std::fmt::Debug + RefUnwindSafe + UnwindSafe {
    type __Static;
}

/// &mut T

#[derive(Debug)]
pub struct __StandRefMut<T>(Box<__Stand<T>>)
where
    T: __Regular;

impl<T> __RegularStand for __StandRefMut<T>
where
    T: __Regular,
{
    type __Static = __StaticRefMut<T>;
}
