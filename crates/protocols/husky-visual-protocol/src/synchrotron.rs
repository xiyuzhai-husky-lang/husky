pub mod action;

use self::action::{VisualSynchrotronAction, VisualSynchrotronActionOutcome};
use crate::visual::{VisualArena, VisualData, VisualId};
use husky_figure_zone_protocol::chunk_base::{
    text::{FigureTextChunkBaseArena, FigureTextChunkBaseData, FigureTextChunkBaseId},
    FigureChunkBase,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSynchrotron {
    visual_arena: VisualArena,
    figure_text_arena: FigureTextChunkBaseArena,
    actions: Vec<VisualSynchrotronAction>,
}

impl VisualSynchrotron {
    pub fn visual_arena(&self) -> &VisualArena {
        &self.visual_arena
    }

    pub fn figure_text_arena(&self) -> &FigureTextChunkBaseArena {
        &self.figure_text_arena
    }
}

impl VisualSynchrotron {
    /// stores the data in the arena and returns a visual id
    pub(crate) fn alloc_visual(&mut self, data: impl Into<VisualData>) -> VisualId {
        match self.take_action(VisualSynchrotronAction::AllocVisual { data: data.into() }) {
            VisualSynchrotronActionOutcome::AllocVisual { visual_id } => visual_id,
            _ => unreachable!(),
        }
    }

    pub fn alloc_figure_text_chunk_base(
        &mut self,
        text: impl Into<FigureTextChunkBaseData>,
    ) -> FigureChunkBase {
        match self.take_action(VisualSynchrotronAction::AllocFigureText { text: text.into() }) {
            VisualSynchrotronActionOutcome::AllocFigureText { figure_text_id } => {
                FigureChunkBase::Text(figure_text_id)
            }
            _ => unreachable!(),
        }
    }
}

impl std::ops::Index<FigureTextChunkBaseId> for VisualSynchrotron {
    type Output = FigureTextChunkBaseData;

    fn index(&self, index: FigureTextChunkBaseId) -> &Self::Output {
        &self.figure_text_arena[index]
    }
}
