use super::*;
use std::ops::ControlFlow;

impl<C, B> Frozen for ControlFlow<C, B>
where
    C: Frozen,
    B: Frozen,
{
    type Thawed = ControlFlow<C::Thawed, B::Thawed>;

    type Slush = ();

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }

    fn serialize_to_value(&self) -> serde_json::Value {
        todo!()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }
}
