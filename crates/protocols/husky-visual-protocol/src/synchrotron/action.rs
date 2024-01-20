use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualSynchrotronAction {
    AllocVisual { data: VisualData },
}

pub(super) enum VisualSynchrotronActionOutcome {
    AllocVisual { visual_id: VisualId },
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSynchrotronStatus {
    actions_len: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualSynchrotronActionsDiff {
    new_actions: Vec<VisualSynchrotronAction>,
}

impl VisualSynchrotron {
    pub(super) fn take_action(
        &mut self,
        action: VisualSynchrotronAction,
    ) -> VisualSynchrotronActionOutcome {
        self.actions.push(action.clone());
        match action {
            VisualSynchrotronAction::AllocVisual { data } => {
                VisualSynchrotronActionOutcome::AllocVisual {
                    visual_id: self.visual_arena.alloc(data),
                }
            }
        }
    }

    pub fn status(&self) -> VisualSynchrotronStatus {
        VisualSynchrotronStatus {
            actions_len: self.actions.len(),
        }
    }

    pub fn actions_diff(
        &self,
        previous_status: VisualSynchrotronStatus,
    ) -> VisualSynchrotronActionsDiff {
        VisualSynchrotronActionsDiff {
            new_actions: self.actions[previous_status.actions_len..].to_vec(),
        }
    }

    pub fn take_actions_diff(&mut self, actions_diff: VisualSynchrotronActionsDiff) {
        for action in actions_diff.new_actions {
            self.take_action(action);
        }
    }
}
