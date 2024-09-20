use super::*;
use frozen::r#ref::FrozenRef;

#[derive(Debug)]
pub struct ThawedRef<T>(*const T)
where
    T: Thawed;

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

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
