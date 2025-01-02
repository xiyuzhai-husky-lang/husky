use crate::{
    expr::VdMirExprArena,
    hint::VdMirHintArena,
    region::{VdMirExprRegionDataMut, VdMirExprRegionDataRef},
    stmt::{VdMirStmtArena, VdMirStmtArenaRef},
    symbol::local_defn::storage::VdMirSymbolLocalDefnStorage,
    tactic::VdMirTacticArena,
};
use eterned::db::EternerDb;

pub struct VdMirHypothesisConstructor<'db> {
    db: &'db EternerDb,
    expr_arena: VdMirExprArena,
    stmt_arena: VdMirStmtArena,
    hint_arena: VdMirHintArena,
    tactic_arena: VdMirTacticArena,
    symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
}

impl<'db> VdMirHypothesisConstructor<'db> {
    pub fn new(
        db: &'db EternerDb,
        expr_arena: VdMirExprArena,
        stmt_arena: VdMirStmtArena,
        hint_arena: VdMirHintArena,
        tactic_arena: VdMirTacticArena,
        symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    ) -> Self {
        Self {
            db,
            expr_arena,
            stmt_arena,
            hint_arena,
            tactic_arena,
            symbol_local_defn_storage,
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
    pub(crate) fn finish(
        self,
    ) -> (
        VdMirExprArena,
        VdMirStmtArena,
        VdMirHintArena,
        VdMirTacticArena,
        VdMirSymbolLocalDefnStorage,
    ) {
        (
            self.expr_arena,
            self.stmt_arena,
            self.hint_arena,
            self.tactic_arena,
            self.symbol_local_defn_storage,
        )
    }
}
