use crate::*;
use husky_value::IsFrozenValue;
use value::Value;

impl IsFrozenValue for Value {
    type Value = Value;

    fn thaw(&self) -> ((), Value) {
        todo!()
    }

    fn present(
        &self,
        value_presenter_cache: &mut husky_value_protocol::presentation::ValuePresenterCache,
        value_presentation_synchrotron: &mut husky_value_protocol::presentation::synchrotron::ValuePresentationSynchrotron,
    ) -> husky_value_protocol::ugly::__ValuePresentation {
        todo!()
    }

    fn visualize(
        &self,
        visual_synchrotron: &mut husky_visual_protocol::ugly::__VisualSynchrotron,
    ) -> husky_visual_protocol::visual::Visual {
        todo!()
    }
}
