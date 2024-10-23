pub mod action;
#[cfg(feature = "egui")]
pub mod egui;
pub mod event;

use self::event::*;
use crate::ast::{TexAstArena, TexAstData, TexAstIdx, TexAstIdxRange};
use time_capsule::{capsule::TimeCapsule, state::IsTimeCapsuleState};

#[derive(Default)]
pub struct TexAstSheet {
    arena: TexAstArena,
}

pub type TexAstSheetTimeCapsule = TimeCapsule<TexAstSheet>;

impl TexAstSheet {
    fn alloc_ast(&mut self, ast_data: TexAstData) -> TexAstIdx {
        self.arena.alloc_one(ast_data)
    }

    fn alloc_asts(&mut self, asts: impl IntoIterator<Item = TexAstData>) -> TexAstIdxRange {
        self.arena.alloc_batch(asts)
    }

    pub fn arena(&self) -> &TexAstArena {
        &self.arena
    }
}

impl IsTimeCapsuleState for TexAstSheet {
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
    // let mut capsule: TexAstSheetTimeCapsule = todo!();
    // todo!()
}
