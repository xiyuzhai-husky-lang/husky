use crate::{
    expr::{LnMirExprArena, LnMirExprData, LnMirExprIdx, LnMirExprIdxRange},
    helpers::fmt::{LnMirExprFormatter, LnMirExprFormatterConfig},
    item_defn::{LnItemDefnArena, LnItemDefnData, LnItemDefnIdxRange},
    stmt::{LnMirStmtArena, LnMirStmtData, LnMirStmtIdx, LnMirStmtIdxRange},
    tactic::{LnMirTacticArena, LnMirTacticData, LnMirTacticIdx, LnMirTacticIdxRange},
};

pub struct LeanHirExprBuilder<'db> {
    db: &'db ::salsa::Db,
    expr_arena: LnMirExprArena,
    stmt_arena: LnMirStmtArena,
    tactic_arena: LnMirTacticArena,
    item_defn_arena: LnItemDefnArena,
}

impl<'db> LeanHirExprBuilder<'db> {
    pub fn new(db: &'db ::salsa::Db) -> Self {
        Self {
            db,
            expr_arena: Default::default(),
            stmt_arena: Default::default(),
            tactic_arena: Default::default(),
            item_defn_arena: Default::default(),
        }
    }
}

impl<'db> LeanHirExprBuilder<'db> {
    pub fn db(&self) -> &'db ::salsa::Db {
        self.db
    }

    pub fn formatter<'a>(&'a self, config: &'a LnMirExprFormatterConfig) -> LnMirExprFormatter<'a> {
        LnMirExprFormatter::new(
            self.expr_arena.as_arena_ref(),
            self.stmt_arena.as_arena_ref(),
            self.tactic_arena.as_arena_ref(),
            self.item_defn_arena.as_arena_ref(),
            config,
            self.db,
        )
    }
}

impl<'db> LeanHirExprBuilder<'db> {
    pub fn alloc_expr(&mut self, data: LnMirExprData) -> LnMirExprIdx {
        self.expr_arena.alloc_one(data)
    }

    pub fn alloc_exprs(
        &mut self,
        data: impl IntoIterator<Item = LnMirExprData>,
    ) -> LnMirExprIdxRange {
        self.expr_arena.alloc_batch(data)
    }

    pub fn alloc_stmt(&mut self, data: LnMirStmtData) -> LnMirStmtIdx {
        self.stmt_arena.alloc_one(data)
    }

    pub fn alloc_stmts(
        &mut self,
        data: impl IntoIterator<Item = LnMirStmtData>,
    ) -> LnMirStmtIdxRange {
        self.stmt_arena.alloc_batch(data)
    }

    pub fn alloc_tactic(&mut self, data: LnMirTacticData) -> LnMirTacticIdx {
        self.tactic_arena.alloc_one(data)
    }

    pub fn alloc_tactics(
        &mut self,
        data: impl IntoIterator<Item = LnMirTacticData>,
    ) -> LnMirTacticIdxRange {
        self.tactic_arena.alloc_batch(data)
    }

    pub fn alloc_item_defns(&mut self, item_defns: Vec<LnItemDefnData>) -> LnItemDefnIdxRange {
        self.item_defn_arena.alloc_batch(item_defns)
    }

    pub fn finish(
        self,
    ) -> (
        LnMirExprArena,
        LnMirStmtArena,
        LnMirTacticArena,
        LnItemDefnArena,
    ) {
        (
            self.expr_arena,
            self.stmt_arena,
            self.tactic_arena,
            self.item_defn_arena,
        )
    }
}
