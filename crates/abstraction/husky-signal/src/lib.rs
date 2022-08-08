use std::rc::Rc;

use vec_like::VecSet;

pub trait Signalable: std::fmt::Debug + PartialEq {}

impl Signalable for bool {}
impl Signalable for str {}

impl Signalable for () {}

impl Signalable for i32 {}
impl Signalable for u32 {}
impl Signalable for usize {}
impl Signalable for f32 {}
impl Signalable for f64 {}
impl<T> Signalable for *const T {}
impl<T> Signalable for Rc<T> where T: Signalable {}

impl<T> Signalable for Option<T> where T: Signalable {}
impl<T, S> Signalable for (T, S)
where
    T: Signalable,
    S: Signalable,
{
}
impl<'a, T> Signalable for &'a T where T: Signalable + ?Sized {}
impl<T> Signalable for [T] where T: Signalable {}
impl<T, const N: usize> Signalable for [T; N] where T: Signalable {}
impl<T> Signalable for Vec<T> where T: Signalable {}
impl<T> Signalable for VecSet<T> where T: Signalable {}

impl<'a> Signalable for String {}
