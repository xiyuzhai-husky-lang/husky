use super::*;

impl<T> Immortal for &'static T
where
    T: ?Sized + std::fmt::Debug + Sync + RefUnwindSafe,
{
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
