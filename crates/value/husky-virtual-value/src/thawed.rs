use crate::*;
use husky_value::IsThawedValue;
use value::Value;

impl IsThawedValue for Value {
    type Value = Value;

    fn r#move(&mut self) -> Self {
        unreachable!()
    }

    fn from_str_literal(str_value: std::sync::Arc<str>) -> Self {
        todo!()
    }

    fn from_enum_index(
        index: usize,
        presenter: husky_value_protocol::presentation::EnumUnitValuePresenter,
    ) -> Self {
        todo!()
    }

    fn to_bool(self) -> bool {
        todo!()
    }

    fn to_usize(self) -> usize {
        todo!()
    }

    fn is_none(self) -> bool {
        todo!()
    }

    fn is_some(self) -> bool {
        todo!()
    }

    fn index(self, index: usize) -> Result<Self, <Self::Value as husky_value::IsValue>::Exception> {
        todo!()
    }

    fn unwrap(self) -> Result<Self, <Self::Value as husky_value::IsValue>::Exception> {
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

    fn freeze(&self) -> <Self::Value as husky_value::IsValue>::FrozenValue {
        todo!()
    }
}
