pub mod action;
pub mod event;

use self::event::*;
use crate::ast::{MathAstArena, MathAstData, MathAstIdx, MathAstIdxRange};
use time_capsule::state::IsTimeCapsuleState;

#[derive(Default)]
pub struct MathAstSheet {
    arena: MathAstArena,
}

impl MathAstSheet {
    fn alloc_ast(&mut self, ast_data: MathAstData) -> MathAstIdx {
        self.arena.alloc_one(ast_data)
    }

    fn alloc_asts(&mut self, asts: impl IntoIterator<Item = MathAstData>) -> MathAstIdxRange {
        self.arena.alloc_batch(asts)
    }
}

impl IsTimeCapsuleState for MathAstSheet {
    type Event = MathAstEvent;

    fn redo(&mut self, event: &Self::Event) {
        todo!()
    }

    fn undo(&mut self, event: &Self::Event) {
        todo!()
    }
}

#[test]
pub(crate) fn math_ast_sheet_time_capsule_works() {
    let mut capsule: MathAstSheetTimeCapsule = todo!();
    todo!()
}
