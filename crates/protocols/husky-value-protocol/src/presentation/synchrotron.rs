pub mod action;

use self::action::{ValuePresentationSynchrotronAction, ValuePresentationSynchrotronActionsDiff};
use super::*;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuePresentationSynchrotron {}

impl ValuePresentationSynchrotron {
    pub fn actions_diff(
        &self,
        value_presentation_synchrotron_status: ValuePresentationSynchrotronStatus,
    ) -> ValuePresentationSynchrotronActionsDiff {
        // ad hoc: todo!()
        ValuePresentationSynchrotronActionsDiff {}
    }

    pub fn status(&self) -> ValuePresentationSynchrotronStatus {
        ValuePresentationSynchrotronStatus {}
    }

    pub fn take_actions_diff(
        &mut self,
        value_presentation_actions_diff: ValuePresentationSynchrotronActionsDiff,
    ) {
        // ad hoc todo!()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuePresentationSynchrotronStatus {}
