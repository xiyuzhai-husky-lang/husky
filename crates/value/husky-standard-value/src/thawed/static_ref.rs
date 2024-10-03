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

impl<T> FromThawedValue for &'static T
where
    T: ?Sized + std::fmt::Debug + Sync + RefUnwindSafe,
{
    #[doc = r" `slush_values` is needed for keeping memory valid when coersing owned ty into ref or ref mut"]
    fn from_thawed_value_aux(value: ThawedValue, slush_values: Option<&mut SlushValues>) -> Self {
        todo!()
    }
}

impl<T> IntoThawedValue for &'static T
where
    T: ?Sized + std::fmt::Debug + Sync + RefUnwindSafe,
{
    fn into_thawed_value(self) -> ThawedValue {
        todo!()
    }
}
