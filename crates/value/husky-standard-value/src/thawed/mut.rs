use super::*;

#[derive(Debug)]
pub struct ThawedMut<T>(*const T)
where
    T: Thawed;

impl<T> Thawed for ThawedMut<T>
where
    T: Thawed,
{
    type Frozen = FrozenMut<T::Frozen>;

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
