use crate::*;
use husky_value::{IsThawedValue, IsValue};
use husky_value_protocol::presentation::ValuePresentation;
use husky_value_protocol::presentation::{
    synchrotron::ValuePresentationSynchrotron, EnumUnitValuePresenter, ValuePresenterCache,
};
use husky_visual_protocol::{synchrotron::VisualSynchrotron, visual::Visual};
use value::Value;

impl IsThawedValue for Value {
    type Value = Value;

    fn r#move(&mut self) -> Self {
        unreachable!()
    }

    fn from_str_literal(str_value: std::sync::Arc<str>) -> Self {
        todo!()
    }

    fn from_enum_index(index: usize, presenter: EnumUnitValuePresenter) -> Self {
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

    fn index(self, index: usize) -> Result<Self, <Self::Value as IsValue>::Exception> {
        todo!()
    }

    fn unwrap(self) -> Result<Self, <Self::Value as IsValue>::Exception> {
        todo!()
    }

    fn present(
        &self,
        value_presenter_cache: &mut ValuePresenterCache,
        value_presentation_synchrotron: &mut ValuePresentationSynchrotron,
    ) -> ValuePresentation {
        todo!()
    }

    fn visualize(&self, visual_synchrotron: &mut VisualSynchrotron) -> Visual {
        todo!()
    }

    fn freeze(&self) -> <Self::Value as IsValue>::FrozenValue {
        todo!()
    }
}
