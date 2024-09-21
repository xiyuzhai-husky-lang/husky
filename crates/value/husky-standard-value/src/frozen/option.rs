use super::*;

impl<T> Frozen for Option<T>
where
    T: Frozen,
{
    type Thawed = Option<T::Thawed>;

    type Slush = T::Slush;

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        // (None,self.as_ref().map(|t|t.revive()))
        match self {
            Some(slf) => {
                let (stand, revived) = slf.thaw();
                (stand, Some(revived))
            }
            None => (None, None),
        }
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        self.as_ref()
            .map(|slf| slf.serialize_to_value())
            .unwrap_or_default()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
