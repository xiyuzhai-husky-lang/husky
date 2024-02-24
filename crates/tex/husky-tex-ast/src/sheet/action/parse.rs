//! after text editing, parse the text into a proper ast
use super::*;

pub struct MathAstParseAction {
    ast_idx: MathAstIdx,
}

impl IsTimeCapsuleAction for MathAstParseAction {
    type Event = MathAstEvent;

    type Outcome = ();

    fn add_to_event_buffer(&self, event: &mut MathAstEventBuffer) {
        todo!()
    }

    fn exec(self, sheet: &mut MathAstSheet) -> Self::Outcome {
        let MathAstData::TextEdit { ref buffer } = sheet.arena[self.ast_idx] else {
            unreachable!()
        };
        todo!()
    }
}
