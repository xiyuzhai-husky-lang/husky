pub mod slice;

use super::*;
use frozen::r#ref::FrozenRef;

#[derive(Debug)]
pub struct ThawedRef<T>(*const T);

impl<T> std::ops::Deref for ThawedRef<T> {
    type Target = *const T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Thawed for ThawedRef<T>
where
    T: Thawed,
{
    type Frozen = FrozenRef<T::Frozen>;

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

impl<T> FromThawedValue for ThawedRef<T>
where
    T: Thawed,
{
    fn from_thawed_value_aux(value: ThawedValue, slush_values: Option<&mut SlushValues>) -> Self {
        ThawedRef(value.into_ref(slush_values))
    }
}

impl<T> IntoThawedValue for ThawedRef<T>
where
    T: Thawed,
{
    fn into_thawed_value(self) -> ThawedValue {
        todo!()
    }
}
