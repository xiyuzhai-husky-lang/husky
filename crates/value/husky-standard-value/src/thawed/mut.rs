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

    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn is_copyable() -> bool {
        todo!()
    }

    fn try_copy(&self) -> Option<Value> {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
