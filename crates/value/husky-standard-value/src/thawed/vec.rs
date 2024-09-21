use super::*;

impl<T> Thawed for Vec<T>
where
    T: Thawed,
{
    type Frozen = Vec<T::Frozen>;

    fn is_copyable() -> bool {
        false
    }

    fn try_copy_thawed(&self) -> Option<ThawedValue> {
        todo!()
    }

    fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn index_ref_thawed<'a>(&'a self, index: usize) -> ExceptedThawedValue {
        Ok(ThawedValue::from_ref(&self[index]))
    }

    fn index_leash_thawed(&'static self, index: usize) -> ExceptedThawedValue {
        Ok(ThawedValue::from_leash(&self[index]))
    }
}
