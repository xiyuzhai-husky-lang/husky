use crate::*;

pub trait __WithEvalLifetime<'eval> {
    type __ThisWithEvalLifetime;
}

impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for &'a T
where
    T::__ThisWithEvalLifetime: 'a,
{
    type __ThisWithEvalLifetime = &'a T::__ThisWithEvalLifetime;
}
impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for Option<T> {
    type __ThisWithEvalLifetime = Option<T::__ThisWithEvalLifetime>;
}

pub trait __Any: __StaticInfo + for<'eval> __WithEvalLifetime<'eval> {}

impl<T: __Any> __Any for Option<T> {}
impl<'a, T: __Any> __Any for &'a T where
    for<'eval> <T as __WithEvalLifetime<'eval>>::__ThisWithEvalLifetime: 'a
{
}
