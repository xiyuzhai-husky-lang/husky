pub mod action;
#[cfg(feature = "egui")]
pub mod egui;
pub mod event;

use self::event::*;
use crate::ast::{LxAstArena, LxAstData, LxAstIdx, LxAstIdxRange};
use time_capsule::{capsule::TimeCapsule, state::IsTimeCapsuleState};

#[derive(Default)]
pub struct LxAstSheet {
    arena: LxAstArena,
}

pub type LxAstSheetTimeCapsule = TimeCapsule<LxAstSheet>;

impl LxAstSheet {
    fn alloc_ast(&mut self, ast_data: LxAstData) -> LxAstIdx {
        self.arena.alloc_one(ast_data)
    }

    fn alloc_asts(&mut self, asts: impl IntoIterator<Item = LxAstData>) -> LxAstIdxRange {
        self.arena.alloc_batch(asts)
    }

    pub fn arena(&self) -> &LxAstArena {
        &self.arena
    }
}

impl IsTimeCapsuleState for LxAstSheet {
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
    // let mut capsule: LxAstSheetTimeCapsule = todo!();
    // todo!()
}
