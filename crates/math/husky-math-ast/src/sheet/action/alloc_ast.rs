use super::*;

pub struct MathAstAllocAstAction {
    ast_data: MathAstData,
}

impl MathAstAllocAstAction {
    pub fn new(ast_data: MathAstData) -> Self {
        Self { ast_data }
    }
}

impl IsTimeCapsuleAction for MathAstAllocAstAction {
    type Event = MathAstEvent;

    type Outcome = MathAstIdx;

    fn add_to_event_buffer(&self, _event: &mut MathAstEventBuffer) {
        // ignored because sheet is append only, no need to undo
        ()
    }

    fn exec(self, sheet: &mut MathAstSheet) -> Self::Outcome {
        sheet.alloc_ast(self.ast_data)
    }
}
