use super::*;
use crate::registry::{
    associated_trace::IsAssociatedTraceRegistry,
    trace_path::{TracePathDisambiguator, TracePathRegistry},
};
use husky_entity_path::PrincipalEntityPath;
use husky_hir_lazy_expr::{
    builder::hir_lazy_expr_region_with_source_map, source_map::HirLazyExprSourceMap,
    HirLazyExprRegion,
};
use husky_hir_lazy_expr::{source_map::HirLazyExprSourceMapData, HirLazyStmtIdx};

use husky_regional_token::{
    ElifRegionalToken, ElseRegionalToken, EolColonRegionalToken, IfRegionalToken,
    RegionalTokenIdxRange,
};
use husky_sema_expr::{
    helpers::range::sema_expr_range_region, SemaExprData, SemaExprDb, SemaExprRegion, SemaStmtData,
    SemaStmtIdx, SemaStmtIdxRange,
};
use husky_token_info::TokenInfoSource;
use husky_val_repr::expansion::ValReprExpansion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(debug_assertions, salsa::debug_with_db(db = TraceDb))]
pub struct LazyStmtTracePathData {
    biological_parent_path: TracePath,
    essence: LazyStmtEssence,
    disambiguator: TracePathDisambiguator<LazyStmtEssence>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyStmtEssence {
    Let,
    Return,
    Require,
    Assert,
    Break,
    Eval,
    IfBranch,
    ElifBranch { elif_branch_idx: u8 },
    ElseBranch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyStmtTraceData {
    path: TracePath,
    biological_parent: TraceId,
    sema_stmt_idx: SemaStmtIdx,
    hir_lazy_stmt_idx: Option<HirLazyStmtIdx>,
    lazy_stmt_sketch: LazyStmtSketch,
    sema_expr_region: SemaExprRegion,
    hir_lazy_expr_region: HirLazyExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyStmtSketch {
    BasicStmt,
    IfBranch {
        if_regional_token: IfRegionalToken,
        eol_colon_regional_token: EolColonRegionalToken,
        stmts: SemaStmtIdxRange,
    },
    ElifBranch {
        elif_branch_idx: u8,
        elif_regional_token: ElifRegionalToken,
        eol_colon_regional_token: EolColonRegionalToken,
        stmts: SemaStmtIdxRange,
    },
    ElseBranch {
        else_regional_token: ElseRegionalToken,
        eol_colon_regional_token: EolColonRegionalToken,
        stmts: SemaStmtIdxRange,
    },
}

impl TraceId {
    pub(crate) fn new_lazy_stmt(
        biological_parent_path: TracePath,
        biological_parent: TraceId,
        essence: LazyStmtEssence,
        registry: &mut crate::registry::trace_path::TracePathRegistry<LazyStmtEssence>,
        sema_stmt_idx: SemaStmtIdx,
        lazy_stmt_sketch: LazyStmtSketch,
        sema_expr_region: SemaExprRegion,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            LazyStmtTracePathData {
                biological_parent_path: biological_parent_path.into(),
                essence,
                disambiguator: registry.issue(essence),
            },
            db,
        );
        let (hir_lazy_expr_region, hir_lazy_expr_source_map) =
            hir_lazy_expr_region_with_source_map(db, sema_expr_region);
        let hir_lazy_stmt_idx = hir_lazy_expr_source_map
            .data(db)
            .sema_to_hir_lazy_stmt_idx(sema_stmt_idx);
        TraceId::new(
            path,
            LazyStmtTraceData {
                path,
                biological_parent: biological_parent.into(),
                sema_stmt_idx,
                hir_lazy_stmt_idx,
                lazy_stmt_sketch,
                sema_expr_region,
                hir_lazy_expr_region,
            }
            .into(),
            db,
        )
    }
}

impl LazyStmtTraceData {
    pub(super) fn view_lines(&self, trace: TraceId, db: &::salsa::Db) -> TraceViewLines {
        let sema_stmt_idx = self.sema_stmt_idx;
        let sema_expr_region = self.sema_expr_region;
        let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
        let region_path = sema_expr_region.path(db);
        let regional_token_idx_range = match self.lazy_stmt_sketch {
            LazyStmtSketch::BasicStmt => sema_expr_range_region.data(db)[sema_stmt_idx],
            LazyStmtSketch::IfBranch {
                if_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                if_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
            LazyStmtSketch::ElifBranch {
                elif_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                elif_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
            LazyStmtSketch::ElseBranch {
                else_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                else_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
        };
        let token_idx_range = regional_token_idx_range
            .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
        let registry = LazyStmtAssociatedTraceRegistry::new(self.path, trace, sema_expr_region, db);
        TraceViewLines::new(region_path.module_path(db), token_idx_range, registry, db)
    }

    pub(super) fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        match self.lazy_stmt_sketch {
            LazyStmtSketch::BasicStmt => false,
            LazyStmtSketch::IfBranch { .. } => true,
            LazyStmtSketch::ElifBranch { .. } => true,
            LazyStmtSketch::ElseBranch { .. } => true,
        }
    }

    pub(super) fn subtraces(&self, trace_id: TraceId, db: &::salsa::Db) -> Vec<TraceId> {
        let biological_parent_path = self.path;
        let biological_parent = trace_id;
        match self.lazy_stmt_sketch {
            LazyStmtSketch::BasicStmt => vec![],
            LazyStmtSketch::IfBranch { stmts, .. }
            | LazyStmtSketch::ElifBranch { stmts, .. }
            | LazyStmtSketch::ElseBranch { stmts, .. } => TraceId::new_lazy_stmts(
                biological_parent_path,
                biological_parent,
                stmts,
                self.sema_expr_region,
                db,
            ),
        }
    }

    pub(super) fn val_repr(&self, trace_id: TraceId, db: &::salsa::Db) -> Option<ValRepr> {
        let val_repr_expansion = trace_val_repr_expansion(db, trace_id);
        val_repr_expansion
            .hir_lazy_stmt_val_repr_map(db)
            .get(self.hir_lazy_stmt_idx?)
            .copied()
    }

    pub(super) fn val_repr_expansion(&self, db: &::salsa::Db) -> ValReprExpansion {
        // todo: handle loops
        self.biological_parent.val_repr_expansion(db)
    }
}

struct LazyStmtAssociatedTraceRegistry<'a> {
    biological_parent_path: TracePath,
    biological_parent: TraceId,
    sema_expr_region: SemaExprRegion,
    hir_lazy_expr_region: HirLazyExprRegion,
    syn_expr_region_data: &'a SynExprRegionData,
    hir_lazy_expr_source_map: HirLazyExprSourceMap,
    hir_lazy_expr_source_map_data: &'a HirLazyExprSourceMapData,
    lazy_expr_trace_path_registry: TracePathRegistry<LazyExprEssence>,
    lazy_expr_traces_issued: VecPairMap<SemaExprIdx, TraceId>,
    lazy_pattern_expr_trace_path_registry: TracePathRegistry<LazyPatternExprEssence>,
    lazy_pattern_expr_traces_issued: VecPairMap<SynPatternExprIdx, TraceId>,
}

impl<'a> LazyStmtAssociatedTraceRegistry<'a> {
    fn new(
        parent_trace_path: TracePath,
        parent_trace: TraceId,
        sema_expr_region: SemaExprRegion,
        db: &'a ::salsa::Db,
    ) -> Self {
        let (hir_lazy_expr_region, hir_lazy_expr_source_map) =
            hir_lazy_expr_region_with_source_map(db, sema_expr_region);
        LazyStmtAssociatedTraceRegistry {
            biological_parent_path: parent_trace_path,
            biological_parent: parent_trace,
            sema_expr_region,
            hir_lazy_expr_region,
            syn_expr_region_data: sema_expr_region.syn_expr_region(db).data(db),
            hir_lazy_expr_source_map,
            hir_lazy_expr_source_map_data: hir_lazy_expr_source_map.data(db),
            lazy_expr_trace_path_registry: Default::default(),
            lazy_expr_traces_issued: Default::default(),
            lazy_pattern_expr_trace_path_registry: Default::default(),
            lazy_pattern_expr_traces_issued: Default::default(),
        }
    }
}

impl<'a> IsAssociatedTraceRegistry for LazyStmtAssociatedTraceRegistry<'a> {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<TraceId> {
        match source {
            TokenInfoSource::UseExpr(_) => None,
            TokenInfoSource::SemaExpr(sema_expr_idx) => Some(
                self.lazy_expr_traces_issued
                    .get_value_copied_or_insert_with(sema_expr_idx, || {
                        let hir_lazy_expr_idx = self
                            .hir_lazy_expr_source_map_data
                            .sema_to_hir_lazy_expr_idx(sema_expr_idx);
                        TraceId::new_lazy_expr(
                            self.biological_parent_path,
                            self.biological_parent,
                            sema_expr_idx,
                            hir_lazy_expr_idx,
                            self.sema_expr_region,
                            self.hir_lazy_expr_region,
                            self.hir_lazy_expr_source_map,
                            &mut self.lazy_expr_trace_path_registry,
                            db,
                        )
                    })
                    .into(),
            ),
            TokenInfoSource::SynPrincipalEntityPathExpr(
                _syn_principal_entity_path_expr_idx,
                syn_principal_entity_path,
            ) => match syn_principal_entity_path {
                PrincipalEntityPath::Module(_) => None,
                PrincipalEntityPath::MajorItem(major_item_path) => {
                    TraceId::from_major_item_path(major_item_path, db)
                }
                PrincipalEntityPath::TypeVariant(_) => None,
            },
            TokenInfoSource::PatternExpr(syn_pattern_expr_idx) => Some(
                self.lazy_pattern_expr_traces_issued
                    .get_value_copied_or_insert_with(syn_pattern_expr_idx, || {
                        TraceId::new_lazy_pattern_expr(
                            self.biological_parent_path,
                            self.biological_parent,
                            syn_pattern_expr_idx,
                            self.hir_lazy_expr_source_map_data
                                .syn_to_hir_lazy_pattern_expr_idx(syn_pattern_expr_idx),
                            self.syn_expr_region_data
                                .syn_pattern_expr_current_syn_symbols_mapped(
                                    syn_pattern_expr_idx,
                                    |current_syn_symbol_idx| {
                                        self.hir_lazy_expr_source_map_data
                                            .current_syn_symbol_to_hir_lazy_variable(
                                                current_syn_symbol_idx,
                                            )
                                    },
                                ),
                            self.sema_expr_region,
                            self.hir_lazy_expr_region,
                            &mut self.lazy_pattern_expr_trace_path_registry,
                            db,
                        )
                    })
                    .into(),
            ),
            TokenInfoSource::TemplateParameter(_) => None,
            TokenInfoSource::AstIdentifiable => None,
        }
    }
}

impl TraceId {
    pub(crate) fn new_lazy_stmts_from_syn_body_with_syn_expr_region(
        parent_trace_path: TracePath,
        parent_trace: TraceId,
        body_with_syn_expr_region: Option<(SynExprIdx, SynExprRegion)>,
        db: &::salsa::Db,
    ) -> Vec<TraceId> {
        let Some((body, syn_expr_region)) = body_with_syn_expr_region else {
            return vec![];
        };
        let sema_expr_region = db.sema_expr_region(syn_expr_region);
        let sema_expr_region_data = sema_expr_region.data(db);
        let body = sema_expr_region_data.syn_root_to_sema_expr_idx(body);
        let SemaExprData::Block { stmts } = *body.data(sema_expr_region_data.sema_expr_arena())
        else {
            unreachable!()
        };
        Self::new_lazy_stmts(parent_trace_path, parent_trace, stmts, sema_expr_region, db)
    }

    pub(crate) fn new_lazy_stmts(
        parent_trace_path: TracePath,
        parent_trace: TraceId,
        stmts: husky_sema_expr::SemaStmtIdxRange,
        sema_expr_region: husky_sema_expr::SemaExprRegion,
        db: &::salsa::Db,
    ) -> Vec<TraceId> {
        let mut registry = TracePathRegistry::<LazyStmtEssence>::default();
        let mut subtraces: Vec<TraceId> = vec![];
        let sema_stmt_arena = sema_expr_region.data(db).sema_stmt_arena();
        for stmt in stmts {
            match stmt.data(sema_stmt_arena) {
                SemaStmtData::Let {
                    let_token: _,
                    let_pattern_sema_obelisk: _,
                    eq_token: _,
                    initial_value_sema_expr_idx: _,
                } => {
                    let lazy_stmt_sketch = LazyStmtEssence::Let {};
                    let lazy_stmt_trace = TraceId::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        lazy_stmt_sketch,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Return {
                    return_token: _,
                    result: _,
                } => {
                    let path_data = LazyStmtEssence::Return {};
                    let lazy_stmt_trace = TraceId::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Require {
                    require_token: _,
                    condition: _,
                } => {
                    let path_data = LazyStmtEssence::Require {};
                    let lazy_stmt_trace = TraceId::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Assert {
                    assert_token: _,
                    condition: _,
                } => {
                    let path_data = LazyStmtEssence::Assert {};
                    let lazy_stmt_trace = TraceId::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Break { break_token: _ } => {
                    let path_data = LazyStmtEssence::Break {};
                    let lazy_stmt_trace = TraceId::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Eval {
                    sema_expr_idx: _,
                    eol_semicolon: _,
                } => {
                    let path_data = LazyStmtEssence::Eval {};
                    let lazy_stmt_trace = TraceId::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::ForBetween {
                    for_token: _,
                    particulars: _,
                    for_loop_var_symbol_idx: _,
                    eol_colon: _,
                    block: _,
                } => todo!(),
                SemaStmtData::ForIn {
                    for_token: _,
                    condition: _,
                    eol_colon: _,
                    block: _,
                } => todo!(),
                SemaStmtData::Forext {
                    forext_token: _,
                    particulars: _,
                    eol_colon: _,
                    block: _,
                } => todo!(),
                SemaStmtData::While {
                    while_token: _,
                    condition: _,
                    eol_colon: _,
                    block: _,
                } => todo!(),
                SemaStmtData::DoWhile {
                    do_token: _,
                    while_token: _,
                    condition: _,
                    eol_colon: _,
                    block: _,
                } => todo!(),
                SemaStmtData::IfElse {
                    sema_if_branch,
                    sema_elif_branches,
                    sema_else_branch,
                } => {
                    subtraces.push(
                        TraceId::new_lazy_stmt(
                            parent_trace_path,
                            parent_trace,
                            LazyStmtEssence::IfBranch,
                            &mut registry,
                            stmt,
                            LazyStmtSketch::IfBranch {
                                if_regional_token: sema_if_branch.if_token(),
                                eol_colon_regional_token: sema_if_branch.eol_colon_token(),
                                stmts: sema_if_branch.stmts(),
                            },
                            sema_expr_region,
                            db,
                        )
                        .into(),
                    );
                    for (elif_branch_idx, sema_elif_branch) in sema_elif_branches.iter().enumerate()
                    {
                        let elif_branch_idx = elif_branch_idx.try_into().unwrap();
                        subtraces.push(
                            TraceId::new_lazy_stmt(
                                parent_trace_path,
                                parent_trace,
                                LazyStmtEssence::ElifBranch { elif_branch_idx },
                                &mut registry,
                                stmt,
                                LazyStmtSketch::ElifBranch {
                                    elif_branch_idx,
                                    elif_regional_token: sema_elif_branch.elif_regional_token(),
                                    eol_colon_regional_token: sema_elif_branch.eol_colon_token(),
                                    stmts: sema_elif_branch.stmts(),
                                },
                                sema_expr_region,
                                db,
                            )
                            .into(),
                        );
                    }
                    if let Some(sema_else_branch) = sema_else_branch {
                        subtraces.push(
                            TraceId::new_lazy_stmt(
                                parent_trace_path,
                                parent_trace,
                                LazyStmtEssence::ElseBranch,
                                &mut registry,
                                stmt,
                                LazyStmtSketch::ElseBranch {
                                    else_regional_token: sema_else_branch.else_regional_token(),
                                    eol_colon_regional_token: sema_else_branch
                                        .eol_colon_regional_token(),
                                    stmts: sema_else_branch.stmts(),
                                },
                                sema_expr_region,
                                db,
                            )
                            .into(),
                        );
                    }
                }
                SemaStmtData::Match {
                    match_token: _,
                    match_target: _,
                    eol_with_token: _,
                    case_branches: _,
                } => todo!(),
            }
        }
        subtraces
    }
}
