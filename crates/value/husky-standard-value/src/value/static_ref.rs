use super::*;

impl<T> Immortal for &'static T
where
    T: ?Sized + std::fmt::Debug + Sync + RefUnwindSafe,
    &'static T: FromValue + IntoValue,
{
    fn try_copy(&self) -> Option<Value> {
        todo!()
    }
}
