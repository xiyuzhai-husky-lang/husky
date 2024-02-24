use super::*;

pub struct MathAstAllocAstAction {
    asts_data: Vec<MathAstData>,
}

impl MathAstAllocAstAction {
    pub fn new(asts_data: Vec<MathAstData>) -> Self {
        Self { asts_data }
    }
}

impl IsTimeCapsuleAction for MathAstAllocAstAction {
    type Event = MathAstEvent;

    type Outcome = MathAstIdxRange;

    fn add_to_event_buffer(&self, _event: &mut MathAstEventBuffer) {
        // ignored because sheet is append only, no need to undo
        ()
    }

    fn exec(self, sheet: &mut MathAstSheet) -> Self::Outcome {
        sheet.alloc_asts(self.asts_data)
    }
}
