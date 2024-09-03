use super::*;

impl<T> Immortal for Vec<T>
where
    T: Immortal,
{
    fn is_copyable() -> bool {
        false
    }

    fn try_copy(&self) -> Option<Value> {
        None
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
