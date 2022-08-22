use crate::*;

pub trait __WithEvalLifetime<'eval> {
    type This;
}

impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for &'a T
where
    T::This: 'a,
{
    type This = &'a T::This;
}
impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for Option<T> {
    type This = Option<T::This>;
}

pub trait __Any: __StaticInfo + for<'eval> __WithEvalLifetime<'eval> {}

impl<T: __Any> __Any for Option<T> {}
impl<'a, T: __Any> __Any for &'a T where for<'eval> <T as __WithEvalLifetime<'eval>>::This: 'a {}
