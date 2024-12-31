use crate::{
    elaboration::{VdMirTacticElaboration, VdMirTracker},
    expr::{VdMirExprArena, VdMirExprArenaRef},
    hint::{VdMirHintArena, VdMirHintArenaRef, VdMirHintIdx},
    stmt::{VdMirStmtArena, VdMirStmtArenaRef, VdMirStmtIdx},
    symbol::local_defn::storage::VdMirSymbolLocalDefnStorage,
    tactic::{VdMirTacticArena, VdMirTacticArenaRef},
};

pub struct VdMirExprRegionData {
    expr_arena: VdMirExprArena,
    stmt_arena: VdMirStmtArena,
    hint_arena: VdMirHintArena,
    tactic_arena: VdMirTacticArena,
    symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
}

impl VdMirExprRegionData {
    pub fn new(
        expr_arena: VdMirExprArena,
        stmt_arena: VdMirStmtArena,
        hint_arena: VdMirHintArena,
        tactic_arena: VdMirTacticArena,
        symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    ) -> Self {
        Self {
            expr_arena,
            stmt_arena,
            hint_arena,
            tactic_arena,
            symbol_local_defn_storage,
        }
    }
}

impl VdMirExprRegionData {
    pub fn expr_arena(&self) -> VdMirExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn stmt_arena(&self) -> VdMirStmtArenaRef {
        self.stmt_arena.as_arena_ref()
    }

    pub fn hint_arena(&self) -> VdMirHintArenaRef {
        self.hint_arena.as_arena_ref()
    }

    pub fn tactic_arena(&self) -> VdMirTacticArenaRef {
        self.tactic_arena.as_arena_ref()
    }

    pub fn symbol_local_defn_storage(&self) -> &VdMirSymbolLocalDefnStorage {
        &self.symbol_local_defn_storage
    }
}

#[derive(Clone, Copy)]
pub struct VdMirExprRegionDataRef<'a> {
    pub expr_arena: VdMirExprArenaRef<'a>,
    pub stmt_arena: VdMirStmtArenaRef<'a>,
    pub hint_arena: VdMirHintArenaRef<'a>,
    pub symbol_local_defn_storage: &'a VdMirSymbolLocalDefnStorage,
}

pub struct VdMirExprRegionDataMut<'a> {
    pub(crate) expr_arena: &'a mut VdMirExprArena,
    pub(crate) stmt_arena: &'a mut VdMirStmtArena,
    pub(crate) hint_arena: &'a mut VdMirHintArena,
    pub(crate) tactic_arena: &'a mut VdMirTacticArena,
    pub(crate) symbol_local_defn_storage: &'a VdMirSymbolLocalDefnStorage,
}

impl<'a> VdMirExprRegionDataMut<'a> {
    pub fn as_region_data_ref(&self) -> VdMirExprRegionDataRef {
        VdMirExprRegionDataRef {
            expr_arena: self.expr_arena.as_arena_ref(),
            stmt_arena: self.stmt_arena.as_arena_ref(),
            hint_arena: self.hint_arena.as_arena_ref(),
            symbol_local_defn_storage: self.symbol_local_defn_storage,
        }
    }

    #[inline(always)]
    pub fn set_elaboration_tracker(
        &mut self,
        stmt: VdMirStmtIdx,
        elaboration_tracker: VdMirTracker,
    ) {
        self.stmt_arena.update(stmt, |entry| {
            entry.set_elaboration_tracker(elaboration_tracker)
        });
    }
}
