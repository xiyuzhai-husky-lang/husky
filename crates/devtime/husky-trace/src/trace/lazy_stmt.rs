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
    helpers::range::sema_expr_range_region, SemaExprRegion, SemaStmtData, SemaStmtIdx,
    SemaStmtIdxRange,
};
use husky_token_info::TokenInfoSource;
use husky_val_repr::expansion::ValReprExpansion;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyStmtTracePath {
    pub parent_path: LazyStmtTraceBiologicalParentPath,
    #[return_ref]
    pub path_data: LazyStmtTracePathData,
    pub disambiguator: TracePathDisambiguator<LazyStmtTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParentPath {
    ValItem(ValItemTracePath),
    LazyCall(LazyCallTracePath),
    LazyStmt(LazyStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LazyStmtTracePathData {
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

impl LazyStmtTracePath {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        path_data: LazyStmtTracePathData,
        registry: &mut TracePathRegistry<LazyStmtTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            biological_parent_path.into(),
            path_data,
            registry.issue(path_data),
        )
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyStmtTrace {
    #[id]
    pub path: LazyStmtTracePath,
    pub biological_parent: LazyStmtTraceBiologicalParent,
    pub sema_stmt_idx: SemaStmtIdx,
    pub hir_lazy_stmt_idx: Option<HirLazyStmtIdx>,
    pub data: LazyStmtTraceData,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_lazy_expr_region: HirLazyExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyStmtTraceData {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParent {
    ValItem(ValItemTrace),
    LazyCall(LazyCallTrace),
    LazyStmt(LazyStmtTrace),
}

impl LazyStmtTrace {
    pub(crate) fn new(
        biological_parent: impl Into<LazyStmtTraceBiologicalParent>,
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        trace_path_data: LazyStmtTracePathData,
        registry: &mut crate::registry::trace_path::TracePathRegistry<LazyStmtTracePathData>,
        sema_stmt_idx: SemaStmtIdx,
        trace_data: LazyStmtTraceData,
        sema_expr_region: SemaExprRegion,
        db: &dyn TraceDb,
    ) -> Self {
        let path = LazyStmtTracePath::new(biological_parent_path, trace_path_data, registry, db);
        let (hir_lazy_expr_region, hir_lazy_expr_source_map) =
            hir_lazy_expr_region_with_source_map(db, sema_expr_region);
        let hir_lazy_stmt_idx = hir_lazy_expr_source_map
            .data(db)
            .sema_to_hir_lazy_stmt_idx(sema_stmt_idx);
        LazyStmtTrace::new_inner(
            db,
            path,
            biological_parent.into(),
            sema_stmt_idx,
            hir_lazy_stmt_idx,
            trace_data,
            sema_expr_region,
            hir_lazy_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = lazy_stmt_trace_view_lines(db, self);
        TraceViewData::new(tokens.data().to_vec(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        match self.data(db) {
            LazyStmtTraceData::BasicStmt => false,
            LazyStmtTraceData::IfBranch { .. } => true,
            LazyStmtTraceData::ElifBranch { .. } => true,
            LazyStmtTraceData::ElseBranch { .. } => true,
        }
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        lazy_stmt_trace_subtraces(db, self)
    }

    pub fn val_repr(self, db: &dyn TraceDb) -> Option<ValRepr> {
        lazy_stmt_trace_val_repr(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_trace_view_lines(db: &dyn TraceDb, trace: LazyStmtTrace) -> TraceViewLines {
    let sema_stmt_idx = trace.sema_stmt_idx(db);
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let region_path = sema_expr_region.path(db);
    let regional_token_idx_range = match trace.data(db) {
        LazyStmtTraceData::BasicStmt => sema_expr_range_region.data(db)[sema_stmt_idx],
        LazyStmtTraceData::IfBranch {
            if_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            if_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
        LazyStmtTraceData::ElifBranch {
            elif_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            elif_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
        LazyStmtTraceData::ElseBranch {
            else_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            else_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
    };
    let token_idx_range =
        regional_token_idx_range.token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    let registry = LazyStmtAssociatedTraceRegistry::new(trace, sema_expr_region, db);
    TraceViewLines::new(region_path.module_path(db), token_idx_range, registry, db)
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_trace_subtraces(db: &dyn TraceDb, trace: LazyStmtTrace) -> Vec<Trace> {
    match trace.data(db) {
        LazyStmtTraceData::BasicStmt => vec![],
        LazyStmtTraceData::IfBranch { stmts, .. }
        | LazyStmtTraceData::ElifBranch { stmts, .. }
        | LazyStmtTraceData::ElseBranch { stmts, .. } => {
            LazyStmtTrace::from_stmts(trace.path(db), trace, stmts, trace.sema_expr_region(db), db)
        }
    }
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_stmt_trace_val_repr(db: &dyn TraceDb, trace: LazyStmtTrace) -> Option<ValRepr> {
    let val_repr_expansion = lazy_stmt_trace_val_repr_expansion(db, trace);
    val_repr_expansion
        .hir_lazy_stmt_val_repr_map(db)
        .get(trace.hir_lazy_stmt_idx(db)?)
        .copied()
}

#[salsa::tracked(jar = TraceJar)]
pub(crate) fn lazy_stmt_trace_val_repr_expansion(
    db: &dyn TraceDb,
    trace: LazyStmtTrace,
) -> ValReprExpansion {
    match trace.biological_parent(db) {
        LazyStmtTraceBiologicalParent::ValItem(trace) => trace.val_repr(db).expansion(db).unwrap(),
        LazyStmtTraceBiologicalParent::LazyCall(_) => todo!(),
        LazyStmtTraceBiologicalParent::LazyStmt(trace) => match trace.biological_parent(db) {
            LazyStmtTraceBiologicalParent::ValItem(parent) => {
                parent.val_repr(db).expansion(db).unwrap()
            }
            LazyStmtTraceBiologicalParent::LazyCall(_) => todo!(),
            LazyStmtTraceBiologicalParent::LazyStmt(parent) => {
                lazy_stmt_trace_val_repr_expansion(db, parent)
            }
        },
    }
}

struct LazyStmtAssociatedTraceRegistry<'a> {
    parent_trace: LazyStmtTrace,
    sema_expr_region: SemaExprRegion,
    hir_lazy_expr_region: HirLazyExprRegion,
    syn_expr_region_data: &'a SynExprRegionData,
    hir_lazy_expr_source_map: HirLazyExprSourceMap,
    hir_lazy_expr_source_map_data: &'a HirLazyExprSourceMapData,
    lazy_expr_trace_path_registry: TracePathRegistry<LazyExprTracePathData>,
    lazy_expr_traces_issued: VecPairMap<SemaExprIdx, LazyExprTrace>,
    lazy_pattern_expr_trace_path_registry: TracePathRegistry<LazyPatternExprTracePathData>,
    lazy_pattern_expr_traces_issued: VecPairMap<SynPatternExprIdx, LazyPatternExprTrace>,
}

impl<'a> LazyStmtAssociatedTraceRegistry<'a> {
    fn new(
        parent_trace: LazyStmtTrace,
        sema_expr_region: SemaExprRegion,
        db: &'a dyn TraceDb,
    ) -> Self {
        let (hir_lazy_expr_region, hir_lazy_expr_source_map) =
            hir_lazy_expr_region_with_source_map(db, sema_expr_region);
        LazyStmtAssociatedTraceRegistry {
            parent_trace,
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
        db: &dyn TraceDb,
    ) -> Option<Trace> {
        match source {
            TokenInfoSource::UseExpr(_) => None,
            TokenInfoSource::SemaExpr(sema_expr_idx) => Some(
                self.lazy_expr_traces_issued
                    .get_value_copied_or_insert_with(sema_expr_idx, || {
                        let hir_lazy_expr_idx = self
                            .hir_lazy_expr_source_map_data
                            .sema_to_hir_lazy_expr_idx(sema_expr_idx);
                        LazyExprTrace::new(
                            self.parent_trace.path(db),
                            self.parent_trace,
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
                    Trace::from_major_item_path(major_item_path, db)
                }
                PrincipalEntityPath::TypeVariant(_) => None,
            },
            TokenInfoSource::PatternExpr(syn_pattern_expr_idx) => Some(
                self.lazy_pattern_expr_traces_issued
                    .get_value_copied_or_insert_with(syn_pattern_expr_idx, || {
                        LazyPatternExprTrace::new(
                            self.parent_trace.path(db),
                            self.parent_trace,
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

impl LazyStmtTrace {
    pub(crate) fn from_stmts(
        parent_trace_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        parent_trace: impl Into<LazyStmtTraceBiologicalParent>,
        stmts: husky_sema_expr::SemaStmtIdxRange,
        sema_expr_region: husky_sema_expr::SemaExprRegion,
        db: &dyn TraceDb,
    ) -> Vec<Trace> {
        let parent_trace = parent_trace.into();
        let parent_trace_path = parent_trace_path.into();
        let mut registry = TracePathRegistry::<LazyStmtTracePathData>::default();
        let mut subtraces: Vec<Trace> = vec![];
        let sema_stmt_arena = sema_expr_region.data(db).sema_stmt_arena();
        for stmt in stmts {
            match stmt.data(sema_stmt_arena) {
                SemaStmtData::Let {
                    let_token: _,
                    let_pattern_sema_obelisk: _,
                    eq_token: _,
                    initial_value_sema_expr_idx: _,
                } => {
                    let path_data = LazyStmtTracePathData::Let {};
                    let lazy_stmt_trace = LazyStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Return {
                    return_token: _,
                    result: _,
                } => {
                    let path_data = LazyStmtTracePathData::Return {};
                    let lazy_stmt_trace = LazyStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Require {
                    require_token: _,
                    condition: _,
                } => {
                    let path_data = LazyStmtTracePathData::Require {};
                    let lazy_stmt_trace = LazyStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Assert {
                    assert_token: _,
                    condition: _,
                } => {
                    let path_data = LazyStmtTracePathData::Assert {};
                    let lazy_stmt_trace = LazyStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Break { break_token: _ } => {
                    let path_data = LazyStmtTracePathData::Break {};
                    let lazy_stmt_trace = LazyStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemaStmtData::Eval {
                    sema_expr_idx: _,
                    eol_semicolon: _,
                } => {
                    let path_data = LazyStmtTracePathData::Eval {};
                    let lazy_stmt_trace = LazyStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtTraceData::BasicStmt,
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
                        LazyStmtTrace::new(
                            parent_trace,
                            parent_trace_path,
                            LazyStmtTracePathData::IfBranch,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::IfBranch {
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
                            LazyStmtTrace::new(
                                parent_trace,
                                parent_trace_path,
                                LazyStmtTracePathData::ElifBranch { elif_branch_idx },
                                &mut registry,
                                stmt,
                                LazyStmtTraceData::ElifBranch {
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
                            LazyStmtTrace::new(
                                parent_trace,
                                parent_trace_path,
                                LazyStmtTracePathData::ElseBranch,
                                &mut registry,
                                stmt,
                                LazyStmtTraceData::ElseBranch {
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
                    match_target_sema_expr_idx: _,
                    eol_with_token: _,
                    sema_case_branches: _,
                } => todo!(),
            }
        }
        subtraces
    }
}
