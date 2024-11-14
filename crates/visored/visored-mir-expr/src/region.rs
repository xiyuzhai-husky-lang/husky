use crate::{
    expr::{VdHirExprArena, VdHirExprArenaRef},
    stmt::{VdHirStmtArena, VdHirStmtArenaRef},
    symbol::local_defn::storage::VdHirSymbolLocalDefnStorage,
};

pub struct VdHirExprRegionData {
    expr_arena: VdHirExprArena,
    stmt_arena: VdHirStmtArena,
    symbol_local_defn_storage: VdHirSymbolLocalDefnStorage,
}

impl VdHirExprRegionData {
    pub fn new(
        expr_arena: VdHirExprArena,
        stmt_arena: VdHirStmtArena,
        symbol_local_defn_storage: VdHirSymbolLocalDefnStorage,
    ) -> Self {
        Self {
            expr_arena,
            stmt_arena,
            symbol_local_defn_storage,
        }
    }
}

impl VdHirExprRegionData {
    pub fn expr_arena(&self) -> VdHirExprArenaRef {
        self.expr_arena.as_arena_ref()
    }

    pub fn stmt_arena(&self) -> VdHirStmtArenaRef {
        self.stmt_arena.as_arena_ref()
    }

    pub fn symbol_local_defn_storage(&self) -> &VdHirSymbolLocalDefnStorage {
        &self.symbol_local_defn_storage
    }
}
