use super::*;

impl<T> Frozen for Vec<T>
where
    T: Frozen,
{
    type Thawed = Vec<T::Thawed>;

    type Slush = Vec<T::Slush>;

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
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
