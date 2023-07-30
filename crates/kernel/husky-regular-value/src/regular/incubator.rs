use super::*;

pub trait __RegularIncubator: std::fmt::Debug + RefUnwindSafe + UnwindSafe + 'static {
    type __Static: __RegularStatic<__Incubator = Self>;

    unsafe fn incubate_box(&mut self) -> Box<dyn __RegularStaticDyn>;
}

/// &mut T

#[derive(Debug)]
pub struct __RegularValueIncubatorRefMut<T>(Box<(T::__Incubator, T)>)
where
    T: __RegularStatic;

impl<T> __RegularIncubator for __RegularValueIncubatorRefMut<T>
where
    T: __RegularStatic,
{
    type __Static = __RegularValueStaticRefMut<T>;

    unsafe fn incubate_box(&mut self) -> Box<dyn __RegularStaticDyn> {
        unreachable!()
    }
}
