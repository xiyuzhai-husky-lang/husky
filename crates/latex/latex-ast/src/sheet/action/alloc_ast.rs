use super::*;

pub(in crate::sheet) struct MathAstAllocAstAction {
    ast_data: TexAstData,
}

impl MathAstAllocAstAction {
    pub(in crate::sheet) fn new(ast_data: TexAstData) -> Self {
        Self { ast_data }
    }
}

impl IsTimeCapsuleAction for MathAstAllocAstAction {
    type Event = MathAstEvent;

    type Outcome = TexAstIdx;

    fn add_to_event_buffer(&self, _event: &mut MathAstEventBuffer) {
        // ignored because sheet is append only, no need to undo
        ()
    }

    fn exec(self, sheet: &mut TexAstSheet) -> Self::Outcome {
        sheet.alloc_ast(self.ast_data)
    }
}
