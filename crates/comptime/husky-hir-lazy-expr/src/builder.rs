use crate::*;
use husky_expr_ty::{ExprTypeRegion, SynExprDisambiguation};
use husky_fluffy_term::FluffyTermBase;
use husky_syn_expr::{SynExprIdx, SynExprRegion, SynExprRegionData, SynStmtIdx};
use salsa::DebugWithDb;

pub struct HirLazyExprBuilder<'a> {
    db: &'a dyn HirLazyExprDb,
    syn_expr_region_data: &'a SynExprRegionData,
    expr_ty_region: &'a ExprTypeRegion,
    expr_arena: HirLazyExprArena,
    stmt_arena: HirLazyStmtArena,
    pattern_expr_arena: HirLazyPatternExprArena,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub fn new(db: &'a dyn HirLazyExprDb, syn_expr_region: SynExprRegion) -> Self {
        Self {
            db,
            syn_expr_region_data: syn_expr_region.data(db),
            expr_ty_region: db.expr_ty_region(syn_expr_region),
            expr_arena: Default::default(),
            stmt_arena: Default::default(),
            pattern_expr_arena: Default::default(),
        }
    }

    pub fn syn_expr_region_data(&self) -> &'a SynExprRegionData {
        self.syn_expr_region_data
    }

    pub(crate) fn alloc_stmts(
        &mut self,
        syn_stmt_indices: Vec<SynStmtIdx>,
        hir_eager_stmts: Vec<HirLazyStmt>,
    ) -> HirLazyStmtIdxRange {
        debug_assert_eq!(syn_stmt_indices.len(), hir_eager_stmts.len());
        // todo: record syn_stmt_indices in source map
        self.stmt_arena.alloc_batch(hir_eager_stmts)
    }

    pub(crate) fn alloc_expr(
        &mut self,
        syn_expr_idx: SynExprIdx,
        hir_lazy_expr: HirLazyExpr,
    ) -> HirLazyExprIdx {
        // todo: record syn_expr_idx in source map
        self.expr_arena.alloc_one(hir_lazy_expr)
    }

    pub(crate) fn alloc_pattern_expr(
        &mut self,
        pattern_expr: HirLazyPatternExpr,
    ) -> HirLazyPatternExprIdx {
        // todo: record in source map
        self.pattern_expr_arena.alloc_one(pattern_expr)
    }

    pub fn db(&self) -> &'a dyn HirLazyExprDb {
        self.db
    }

    pub(crate) fn path(&self) -> String {
        format!("{:?}", self.syn_expr_region_data.path().debug(self.db))
    }

    #[track_caller]
    pub(crate) fn expr_disambiguation(
        &self,
        syn_expr_idx: SynExprIdx,
    ) -> &'a SynExprDisambiguation {
        let Some(Ok(disambiguation)) = self.expr_ty_region.expr_disambiguation(syn_expr_idx) else {
            unreachable!(
                r#"
    syn_expr = {:?},
    path = {:?},
    self.expr_ty_region.expr_disambiguation(syn_expr_idx) = {:#?}"#,
                self.syn_expr_region_data[syn_expr_idx].debug(self.db),
                self.path(),
                self.expr_ty_region
                    .expr_disambiguation(syn_expr_idx)
                    .debug(self.db)
            )
        };
        disambiguation
    }

    pub(crate) fn expr_term(&self, syn_expr_idx: SynExprIdx) -> EtherealTerm {
        // ad hoc
        match self
            .expr_ty_region
            .expr_fluffy_term(syn_expr_idx)
            .expect("hir stage some")
            .expect("hir stage ok")
            .base_resolved_inner(self.expr_ty_region.fluffy_term_region().terms())
        {
            FluffyTermBase::Ethereal(term) => term,
            FluffyTermBase::Solid(_) => todo!(),
            FluffyTermBase::Hollow(_) => todo!(),
            FluffyTermBase::Place => todo!(),
        }
    }

    pub fn finish(self) -> HirLazyExprRegion {
        HirLazyExprRegion::new(
            self.db,
            self.expr_arena,
            self.stmt_arena,
            self.pattern_expr_arena,
        )
    }
}
