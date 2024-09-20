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

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        let elements = self
            .iter()
            .map(|t| t.visualize_or_void(visual_synchrotron))
            .collect();
        Visual::new_group_visual(elements, visual_synchrotron)
    }
}
