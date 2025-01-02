use crate::{
    expr::{VdMirExprArena, VdMirExprIdx},
    hint::VdMirHintArena,
    hypothesis::VdMirHypothesisEntry,
    region::VdMirExprRegionDataRef,
    stmt::{VdMirStmtArena, VdMirStmtArenaRef},
    symbol::local_defn::storage::VdMirSymbolLocalDefnStorage,
};
use eterned::db::EternerDb;

use super::{construction::VdMirHypothesisConstruction, VdMirHypothesisArena, VdMirHypothesisIdx};

pub struct VdMirHypothesisConstructor<'db> {
    db: &'db EternerDb,
    expr_arena: VdMirExprArena,
    stmt_arena: VdMirStmtArena,
    hint_arena: VdMirHintArena,
    hypothesis_arena: VdMirHypothesisArena,
    symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn new(
        db: &'db EternerDb,
        expr_arena: VdMirExprArena,
        stmt_arena: VdMirStmtArena,
        hint_arena: VdMirHintArena,
        symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    ) -> Self {
        Self {
            db,
            expr_arena,
            stmt_arena,
            hint_arena,
            symbol_local_defn_storage,
            hypothesis_arena: Default::default(),
        }
    }
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn stmt_arena(&self) -> VdMirStmtArenaRef {
        self.stmt_arena.as_arena_ref()
    }

    pub fn stmt_arena_mut(&mut self) -> &mut VdMirStmtArena {
        &mut self.stmt_arena
    }

    pub fn region_data(&self) -> VdMirExprRegionDataRef {
        VdMirExprRegionDataRef {
            expr_arena: self.expr_arena.as_arena_ref(),
            stmt_arena: self.stmt_arena.as_arena_ref(),
            hint_arena: self.hint_arena.as_arena_ref(),
            symbol_local_defn_storage: &self.symbol_local_defn_storage,
        }
    }
}

impl<'db> VdMirHypothesisConstructor<'db> {
    // TODO: do more things like handle hypothesis stack, register src, etc.
    pub fn construct_new_hypothesis(
        &mut self,
        expr: VdMirExprIdx,
        hypothesis: VdMirHypothesisConstruction,
    ) -> VdMirHypothesisIdx {
        self.hypothesis_arena
            .alloc_one(VdMirHypothesisEntry::new(expr, hypothesis))
    }

    pub(crate) fn finish(
        self,
    ) -> (
        VdMirExprArena,
        VdMirStmtArena,
        VdMirHintArena,
        VdMirHypothesisArena,
        VdMirSymbolLocalDefnStorage,
    ) {
        (
            self.expr_arena,
            self.stmt_arena,
            self.hint_arena,
            self.hypothesis_arena,
            self.symbol_local_defn_storage,
        )
    }
}
