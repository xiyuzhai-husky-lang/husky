use super::*;

impl<T> Thawed for &'static T
where
    T: ?Sized + std::fmt::Debug + Sync + RefUnwindSafe,
{
    type Frozen = Self;

    fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn is_copyable() -> bool {
        todo!()
    }

    fn try_copy_thawed(&self) -> Option<ThawedValue> {
        todo!()
    }
}
