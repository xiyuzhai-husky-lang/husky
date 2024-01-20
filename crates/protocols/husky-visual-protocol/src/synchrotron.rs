pub mod action;

use self::action::{VisualSynchrotronAction, VisualSynchrotronActionsDiff};
use crate::visual::VisualArena;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSynchrotron {
    visual_arena: VisualArena,
    actions: Vec<VisualSynchrotronAction>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSynchrotronStatus {}

impl VisualSynchrotron {
    pub fn status(&self) -> VisualSynchrotronStatus {
        // ad hoc
        VisualSynchrotronStatus {}
    }

    pub fn actions_diff(
        &self,
        previous_status: VisualSynchrotronStatus,
    ) -> VisualSynchrotronActionsDiff {
        // ad hoc
        VisualSynchrotronActionsDiff {}
    }

    pub fn take_actions_diff(&mut self, actions_diff: VisualSynchrotronActionsDiff) {
        // todo!()
    }

    pub fn visual_arena(&self) -> &VisualArena {
        &self.visual_arena
    }
}
