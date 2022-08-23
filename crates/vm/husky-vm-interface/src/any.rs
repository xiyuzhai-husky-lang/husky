use crate::*;

pub trait __WithEvalLifetime<'eval> {
    type __SelfWithEvalLifetime;
}

pub trait __Any: __StaticInfo + for<'eval> __WithEvalLifetime<'eval> {}

// &'a T
impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for &'a T
where
    T::__SelfWithEvalLifetime: 'a,
{
    type __SelfWithEvalLifetime = &'a T::__SelfWithEvalLifetime;
}
impl<'a, T: __Any> __Any for &'a T where
    for<'eval> <T as __WithEvalLifetime<'eval>>::__SelfWithEvalLifetime: 'a
{
}

// &'a mut T
impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for &'a mut T
where
    T::__SelfWithEvalLifetime: 'a,
{
    type __SelfWithEvalLifetime = &'a mut T::__SelfWithEvalLifetime;
}
impl<'a, T: __Any> __Any for &'a mut T where
    for<'eval> <T as __WithEvalLifetime<'eval>>::__SelfWithEvalLifetime: 'a
{
}

// Option
impl<'eval, 'a, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for Option<T> {
    type __SelfWithEvalLifetime = Option<T::__SelfWithEvalLifetime>;
}
impl<T: __Any> __Any for Option<T> {}

// Vec
impl<'eval, T: __WithEvalLifetime<'eval>> __WithEvalLifetime<'eval> for Vec<T> {
    type __SelfWithEvalLifetime = Option<T::__SelfWithEvalLifetime>;
}
impl<T: __Any> __Any for Vec<T> {}
