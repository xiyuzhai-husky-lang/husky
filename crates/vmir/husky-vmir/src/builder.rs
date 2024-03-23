use crate::{
    destroyer::{VmirDestroyerArena, VmirDestroyerData, VmirDestroyerIdxRange},
    expr::{VmirExprArena, VmirExprData, VmirExprIdx, VmirExprIdxRange},
    pattern::{
        destructive::{
            VmirDestructivePatternArena, VmirDestructivePatternData, VmirDestructivePatternIdx,
            VmirDestructivePatternIdxRange,
        },
        restructive::{
            VmirRestructivePatternArena, VmirRestructivePatternData, VmirRestructivePatternIdx,
            VmirRestructivePatternIdxRange,
        },
    },
    stmt::{VmirStmtArena, VmirStmtData, VmirStmtIdxRange},
};
use husky_hir_eager_expr::{
    HirEagerExprArena, HirEagerExprIdx, HirEagerPatternArena, HirEagerStmtArena,
};
use husky_hir_expr::{HirExprIdx, HirExprRegion};
use husky_linkage::{instantiation::LinInstantiation, linkage::Linkage};
use husky_task::linktime::IsLinktime;
use idx_arena::ArenaIdx;

pub(crate) struct VmirBuilder<'comptime, Linktime: IsLinktime> {
    db: &'comptime ::salsa::Db,
    hir_eager_expr_arena: &'comptime HirEagerExprArena,
    hir_eager_stmt_arena: &'comptime HirEagerStmtArena,
    hir_eager_pattern_arena: &'comptime HirEagerPatternArena,
    instantiation: &'comptime LinInstantiation,
    linktime: &'comptime Linktime,
    vmir_expr_arena: VmirExprArena<Linktime::LinkageImpl>,
    vmir_stmt_arena: VmirStmtArena<Linktime::LinkageImpl>,
    vmir_restructive_pattern_arena: VmirRestructivePatternArena<Linktime::LinkageImpl>,
    vmir_destructive_pattern_arena: VmirDestructivePatternArena<Linktime::LinkageImpl>,
    vmir_destroyer_arena: VmirDestroyerArena,
}

/// # constructor
impl<'db, Linktime: IsLinktime> VmirBuilder<'db, Linktime> {
    pub(crate) fn new(
        linkage: Linkage,
        db: &'db ::salsa::Db,
        linktime: &'db Linktime,
    ) -> Option<(HirEagerExprIdx, Self)> {
        use husky_hir_defn::defn::HasHirDefn;

        let (path, instantiation) = linkage.path_and_instantiation_for_definition(db)?;
        let hir_defn = path.hir_defn(db).unwrap();
        let (HirExprIdx::Eager(body), HirExprRegion::Eager(hir_eager_expr_region)) =
            hir_defn.hir_expr_body_and_region(db)?
        else {
            return None;
        };
        Some((
            body,
            Self {
                db,
                hir_eager_expr_arena: hir_eager_expr_region.expr_arena(db),
                hir_eager_stmt_arena: hir_eager_expr_region.stmt_arena(db),
                hir_eager_pattern_arena: hir_eager_expr_region.pattern_arena(db),
                instantiation,
                linktime,
                vmir_expr_arena: Default::default(),
                vmir_stmt_arena: Default::default(),
                vmir_restructive_pattern_arena: Default::default(),
                vmir_destructive_pattern_arena: Default::default(),
                vmir_destroyer_arena: Default::default(),
            },
        ))
    }
}

/// # getters
impl<'db, Linktime: IsLinktime> VmirBuilder<'db, Linktime> {
    pub(crate) fn db(&self) -> &'db ::salsa::Db {
        self.db
    }

    pub(crate) fn hir_eager_expr_arena(&self) -> &'db HirEagerExprArena {
        self.hir_eager_expr_arena
    }

    pub(crate) fn hir_eager_stmt_arena(&self) -> &'db HirEagerStmtArena {
        self.hir_eager_stmt_arena
    }

    pub(crate) fn hir_eager_pattern_arena(&self) -> &'db HirEagerPatternArena {
        self.hir_eager_pattern_arena
    }

    pub(crate) fn lin_instantiation(&self) -> &'db LinInstantiation {
        self.instantiation
    }

    pub(crate) fn linkage_impl(&self, linkage: Linkage) -> Linktime::LinkageImpl {
        self.linktime.linkage_impl(linkage, self.db)
    }
}

/// # actions
impl<'db, Linktime: IsLinktime> VmirBuilder<'db, Linktime> {
    pub(crate) fn alloc_expr(
        &mut self,
        expr: VmirExprData<Linktime::LinkageImpl>,
    ) -> ArenaIdx<VmirExprData<Linktime::LinkageImpl>> {
        self.vmir_expr_arena.alloc_one(expr)
    }

    pub(crate) fn alloc_exprs(
        &mut self,
        exprs: Vec<VmirExprData<Linktime::LinkageImpl>>,
    ) -> VmirExprIdxRange<Linktime::LinkageImpl> {
        self.vmir_expr_arena.alloc_batch(exprs)
    }

    pub(crate) fn alloc_stmts(
        &mut self,
        stmts: Vec<VmirStmtData<Linktime::LinkageImpl>>,
    ) -> VmirStmtIdxRange<Linktime::LinkageImpl> {
        self.vmir_stmt_arena.alloc_batch(stmts)
    }

    pub(crate) fn alloc_restructive_pattern(
        &mut self,
        pattern: VmirRestructivePatternData<Linktime::LinkageImpl>,
    ) -> VmirRestructivePatternIdx<Linktime::LinkageImpl> {
        self.vmir_restructive_pattern_arena.alloc_one(pattern)
    }

    pub(crate) fn alloc_restructive_patterns(
        &mut self,
        patterns: Vec<VmirRestructivePatternData<Linktime::LinkageImpl>>,
    ) -> VmirRestructivePatternIdxRange<Linktime::LinkageImpl> {
        self.vmir_restructive_pattern_arena.alloc_batch(patterns)
    }

    pub(crate) fn alloc_destructive_pattern(
        &mut self,
        pattern: VmirDestructivePatternData<Linktime::LinkageImpl>,
    ) -> VmirDestructivePatternIdx<Linktime::LinkageImpl> {
        self.vmir_destructive_pattern_arena.alloc_one(pattern)
    }

    pub(crate) fn alloc_destructive_patterns(
        &mut self,
        patterns: Vec<VmirDestructivePatternData<Linktime::LinkageImpl>>,
    ) -> VmirDestructivePatternIdxRange<Linktime::LinkageImpl> {
        self.vmir_destructive_pattern_arena.alloc_batch(patterns)
    }

    pub(crate) fn alloc_destroyers(
        &mut self,
        destroyer_datas: Vec<VmirDestroyerData>,
    ) -> VmirDestroyerIdxRange {
        self.vmir_destroyer_arena.alloc_batch(destroyer_datas)
    }

    pub(crate) fn finish(
        self,
    ) -> (
        VmirExprArena<Linktime::LinkageImpl>,
        VmirStmtArena<Linktime::LinkageImpl>,
    ) {
        (self.vmir_expr_arena, self.vmir_stmt_arena)
    }
}
