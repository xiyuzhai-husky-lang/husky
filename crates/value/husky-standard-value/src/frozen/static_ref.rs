use super::*;

impl<T> Frozen for &'static T
where
    T: ?Sized + std::fmt::Debug + Sync + RefUnwindSafe,
{
    type Thawed = Self;

    type Slush = ();

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        (None, *self)
    }
}
