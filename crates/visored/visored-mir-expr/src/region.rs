use crate::{
    expr::{VdMirExprArena, VdMirExprArenaRef},
    stmt::{VdMirStmtArena, VdMirStmtArenaRef},
    symbol::local_defn::storage::VdMirSymbolLocalDefnStorage,
    tactic::{VdMirTacticArena, VdMirTacticArenaRef},
};

pub struct VdMirExprRegionData {
    expr_arena: VdMirExprArena,
    stmt_arena: VdMirStmtArena,
    tactic_arena: VdMirTacticArena,
    symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
}

impl VdMirExprRegionData {
    pub fn new(
        expr_arena: VdMirExprArena,
        stmt_arena: VdMirStmtArena,
        tactic_arena: VdMirTacticArena,
        symbol_local_defn_storage: VdMirSymbolLocalDefnStorage,
    ) -> Self {
        Self {
            expr_arena,
            stmt_arena,
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

    pub fn tactic_arena(&self) -> VdMirTacticArenaRef {
        self.tactic_arena.as_arena_ref()
    }

    pub fn symbol_local_defn_storage(&self) -> &VdMirSymbolLocalDefnStorage {
        &self.symbol_local_defn_storage
    }
}

pub struct VdMirExprRegionDataRef<'a> {
    pub expr_arena: VdMirExprArenaRef<'a>,
    pub stmt_arena: VdMirStmtArenaRef<'a>,
    pub symbol_local_defn_storage: &'a VdMirSymbolLocalDefnStorage,
}

pub struct VdMirExprRegionDataMut<'a> {
    pub(crate) expr_arena: &'a mut VdMirExprArena,
    pub(crate) tactic_arena: &'a mut VdMirTacticArena,
}
