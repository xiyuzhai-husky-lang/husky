pub mod action;

use self::action::{
    VisualSynchrotronAction, VisualSynchrotronActionOutcome, VisualSynchrotronActionsDiff,
    VisualSynchrotronStatus,
};
use crate::visual::{VisualArena, VisualData, VisualId};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSynchrotron {
    visual_arena: VisualArena,
    actions: Vec<VisualSynchrotronAction>,
}

impl VisualSynchrotron {
    pub fn visual_arena(&self) -> &VisualArena {
        &self.visual_arena
    }

    /// stores the data in the arena and returns a visual id
    pub(crate) fn alloc_visual(&mut self, data: impl Into<VisualData>) -> VisualId {
        match self.take_action(VisualSynchrotronAction::AllocVisual { data: data.into() }) {
            VisualSynchrotronActionOutcome::AllocVisual { visual_id } => visual_id,
            _ => unreachable!(),
        }
    }
}
