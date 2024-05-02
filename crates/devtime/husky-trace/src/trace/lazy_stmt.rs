use super::*;
use crate::registry::{
    assoc_trace::IsAssocTraceRegistry,
    trace_path::{TracePathDisambiguator, TracePathRegistry},
};
use husky_entity_path::path::PrincipalEntityPath;
use husky_hir_lazy_expr::{
    builder::hir_lazy_expr_region_with_source_map, source_map::HirLazyExprSourceMap,
    HirLazyExprRegion,
};
use husky_hir_lazy_expr::{source_map::HirLazyExprSourceMapData, HirLazyStmtIdx};

use husky_ki_repr::expansion::KiReprExpansion;
use husky_regional_token::{
    ElifRegionalToken, ElseRegionalToken, EolColonRegionalToken, IfRegionalToken,
    RegionalTokenIdxRange,
};
use husky_sem_expr::{
    helpers::range::sem_expr_range_region, SemExprData, SemExprDb, SemExprRegion, SemStmtData,
    SemStmtIdx, SemStmtIdxRange,
};
use husky_syn_defn::ItemSynDefn;
use husky_token_info::TokenInfoSource;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyStmtTraceData {
    path: TracePath,
    biological_parent: Trace,
    sem_stmt_idx: SemStmtIdx,
    hir_lazy_stmt_idx: Option<HirLazyStmtIdx>,
    lazy_stmt_sketch: LazyStmtSketch,
    #[skip_fmt]
    sem_expr_region: SemExprRegion,
    #[skip_fmt]
    hir_lazy_expr_region: HirLazyExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LazyStmtSketch {
    BasicStmt,
    IfBranch {
        if_regional_token: IfRegionalToken,
        eol_colon_regional_token: EolColonRegionalToken,
        stmts: SemStmtIdxRange,
    },
    ElifBranch {
        elif_branch_idx: u8,
        elif_regional_token: ElifRegionalToken,
        eol_colon_regional_token: EolColonRegionalToken,
        stmts: SemStmtIdxRange,
    },
    ElseBranch {
        else_regional_token: ElseRegionalToken,
        eol_colon_regional_token: EolColonRegionalToken,
        stmts: SemStmtIdxRange,
    },
}

impl Trace {
    pub(crate) fn new_lazy_stmt(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        essence: LazyStmtEssence,
        registry: &mut crate::registry::trace_path::TracePathRegistry<LazyStmtEssence>,
        sem_stmt_idx: SemStmtIdx,
        lazy_stmt_sketch: LazyStmtSketch,
        sem_expr_region: SemExprRegion,
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
            hir_lazy_expr_region_with_source_map(db, sem_expr_region);
        let hir_lazy_stmt_idx = hir_lazy_expr_source_map
            .data(db)
            .sem_to_hir_lazy_stmt_idx(sem_stmt_idx);
        Trace::new(
            path,
            LazyStmtTraceData {
                path,
                biological_parent: biological_parent.into(),
                sem_stmt_idx,
                hir_lazy_stmt_idx,
                lazy_stmt_sketch,
                sem_expr_region,
                hir_lazy_expr_region,
            }
            .into(),
            db,
        )
    }
}

impl LazyStmtTraceData {
    pub(super) fn view_lines(&self, trace: Trace, db: &::salsa::Db) -> TraceViewLines {
        let sem_stmt_idx = self.sem_stmt_idx;
        let sem_expr_region = self.sem_expr_region;
        let sem_expr_range_region = sem_expr_range_region(db, sem_expr_region);
        let region_path = sem_expr_region.path(db);
        let regional_token_idx_range = match self.lazy_stmt_sketch {
            LazyStmtSketch::BasicStmt => sem_expr_range_region.data(db)[sem_stmt_idx],
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
        let registry = LazyStmtAssocTraceRegistry::new(self.path, trace, sem_expr_region, db);
        TraceViewLines::new(region_path.module_path(db), token_idx_range, registry, db)
    }

    pub(super) fn have_subtraces(&self, _db: &::salsa::Db) -> bool {
        match self.lazy_stmt_sketch {
            LazyStmtSketch::BasicStmt => false,
            LazyStmtSketch::IfBranch { .. } => true,
            LazyStmtSketch::ElifBranch { .. } => true,
            LazyStmtSketch::ElseBranch { .. } => true,
        }
    }

    pub(super) fn subtraces(&self, trace_id: Trace, db: &::salsa::Db) -> Vec<Trace> {
        let biological_parent_path = self.path;
        let biological_parent = trace_id;
        match self.lazy_stmt_sketch {
            LazyStmtSketch::BasicStmt => vec![],
            LazyStmtSketch::IfBranch { stmts, .. }
            | LazyStmtSketch::ElifBranch { stmts, .. }
            | LazyStmtSketch::ElseBranch { stmts, .. } => Trace::new_lazy_stmts(
                biological_parent_path,
                biological_parent,
                stmts,
                self.sem_expr_region,
                db,
            ),
        }
    }

    pub(super) fn ki_repr(&self, trace: Trace, db: &::salsa::Db) -> Option<KiRepr> {
        let ki_repr_expansion = trace_ki_repr_expansion(db, trace);
        ki_repr_expansion
            .hir_lazy_stmt_ki_repr_map(db)
            .get(self.hir_lazy_stmt_idx?)
            .copied()
    }

    pub(super) fn ki_repr_expansion(&self, db: &::salsa::Db) -> KiReprExpansion {
        // todo: handle loops
        self.biological_parent.ki_repr_expansion(db)
    }
}

struct LazyStmtAssocTraceRegistry<'a> {
    biological_parent_path: TracePath,
    biological_parent: Trace,
    sem_expr_region: SemExprRegion,
    hir_lazy_expr_region: HirLazyExprRegion,
    syn_expr_region_data: &'a SynExprRegionData,
    hir_lazy_expr_source_map: HirLazyExprSourceMap,
    hir_lazy_expr_source_map_data: &'a HirLazyExprSourceMapData,
    lazy_expr_trace_path_registry: TracePathRegistry<LazyExprEssence>,
    lazy_expr_traces_issued: VecPairMap<SemExprIdx, Trace>,
    lazy_pattern_expr_trace_path_registry: TracePathRegistry<LazyPatternExprEssence>,
    lazy_pattern_expr_traces_issued: VecPairMap<SynPatternIdx, Trace>,
}

impl<'a> LazyStmtAssocTraceRegistry<'a> {
    fn new(
        parent_trace_path: TracePath,
        parent_trace: Trace,
        sem_expr_region: SemExprRegion,
        db: &'a ::salsa::Db,
    ) -> Self {
        let (hir_lazy_expr_region, hir_lazy_expr_source_map) =
            hir_lazy_expr_region_with_source_map(db, sem_expr_region);
        LazyStmtAssocTraceRegistry {
            biological_parent_path: parent_trace_path,
            biological_parent: parent_trace,
            sem_expr_region,
            hir_lazy_expr_region,
            syn_expr_region_data: sem_expr_region.syn_expr_region(db).data(db),
            hir_lazy_expr_source_map,
            hir_lazy_expr_source_map_data: hir_lazy_expr_source_map.data(db),
            lazy_expr_trace_path_registry: Default::default(),
            lazy_expr_traces_issued: Default::default(),
            lazy_pattern_expr_trace_path_registry: Default::default(),
            lazy_pattern_expr_traces_issued: Default::default(),
        }
    }
}

impl<'a> IsAssocTraceRegistry for LazyStmtAssocTraceRegistry<'a> {
    fn get_or_issue_assoc_trace(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<Trace> {
        match source {
            TokenInfoSource::UseExpr(_) => None,
            TokenInfoSource::SemExpr(sem_expr_idx) => Some(
                self.lazy_expr_traces_issued
                    .get_value_copied_or_insert_with(sem_expr_idx, || {
                        let hir_lazy_expr_idx = self
                            .hir_lazy_expr_source_map_data
                            .sem_to_hir_lazy_expr_idx(sem_expr_idx);
                        Trace::new_lazy_expr(
                            self.biological_parent_path,
                            self.biological_parent,
                            sem_expr_idx,
                            hir_lazy_expr_idx,
                            self.sem_expr_region,
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
                        Trace::new_lazy_pattern_expr(
                            self.biological_parent_path,
                            self.biological_parent,
                            syn_pattern_expr_idx,
                            self.hir_lazy_expr_source_map_data
                                .syn_to_hir_lazy_pattern_expr_idx(syn_pattern_expr_idx),
                            self.syn_expr_region_data
                                .syn_pattern_expr_current_variables_mapped(
                                    syn_pattern_expr_idx,
                                    |current_variable_idx| {
                                        self.hir_lazy_expr_source_map_data
                                            .current_variable_to_hir_lazy_variable(
                                                current_variable_idx,
                                            )
                                    },
                                ),
                            self.sem_expr_region,
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

impl Trace {
    pub(crate) fn new_lazy_stmts_from_syn_body_with_syn_expr_region(
        parent_trace_path: TracePath,
        parent_trace: Trace,
        syn_defn: Option<ItemSynDefn>,
        db: &::salsa::Db,
    ) -> Vec<Trace> {
        let Some(ItemSynDefn {
            body,
            syn_expr_region,
        }) = syn_defn
        else {
            return vec![];
        };
        let sem_expr_region = db.sem_expr_region(syn_expr_region);
        let sem_expr_region_data = sem_expr_region.data(db);
        let body = sem_expr_region_data.syn_root_to_sem_expr_idx(body);
        let SemExprData::Block { stmts } = *body.data(sem_expr_region_data.sem_expr_arena()) else {
            unreachable!()
        };
        Self::new_lazy_stmts(parent_trace_path, parent_trace, stmts, sem_expr_region, db)
    }

    pub(crate) fn new_lazy_stmts(
        parent_trace_path: TracePath,
        parent_trace: Trace,
        stmts: husky_sem_expr::SemStmtIdxRange,
        sem_expr_region: husky_sem_expr::SemExprRegion,
        db: &::salsa::Db,
    ) -> Vec<Trace> {
        let mut registry = TracePathRegistry::<LazyStmtEssence>::default();
        let mut subtraces: Vec<Trace> = vec![];
        let sem_stmt_arena = sem_expr_region.data(db).sem_stmt_arena();
        for stmt in stmts {
            match stmt.data(sem_stmt_arena) {
                SemStmtData::Let { .. } => {
                    let lazy_stmt_sketch = LazyStmtEssence::Let {};
                    let lazy_stmt_trace = Trace::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        lazy_stmt_sketch,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sem_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemStmtData::Return { .. } => {
                    let path_data = LazyStmtEssence::Return {};
                    let lazy_stmt_trace = Trace::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sem_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemStmtData::Require {
                    require_token: _,
                    condition: _,
                } => {
                    let path_data = LazyStmtEssence::Require {};
                    let lazy_stmt_trace = Trace::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sem_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemStmtData::Assert {
                    assert_token: _,
                    condition: _,
                } => {
                    let path_data = LazyStmtEssence::Assert {};
                    let lazy_stmt_trace = Trace::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sem_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemStmtData::Break { break_token: _ } => {
                    let path_data = LazyStmtEssence::Break {};
                    let lazy_stmt_trace = Trace::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sem_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemStmtData::Eval { .. } => {
                    let path_data = LazyStmtEssence::Eval {};
                    let lazy_stmt_trace = Trace::new_lazy_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        LazyStmtSketch::BasicStmt,
                        sem_expr_region,
                        db,
                    );
                    subtraces.push(lazy_stmt_trace.into())
                }
                SemStmtData::ForBetween {
                    for_token: _,
                    particulars: _,
                    for_loop_var_symbol_idx: _,
                    eol_colon: _,
                    stmts: _,
                } => todo!(),
                SemStmtData::ForIn {
                    for_token: _,
                    range: _,
                    eol_colon: _,
                    stmts: _,
                } => todo!(),
                SemStmtData::Forext {
                    forext_token: _,
                    particulars: _,
                    eol_colon: _,
                    stmts: _,
                } => todo!(),
                SemStmtData::While {
                    while_token: _,
                    condition: _,
                    eol_colon: _,
                    stmts: _,
                } => todo!(),
                SemStmtData::DoWhile {
                    do_token: _,
                    while_token: _,
                    condition: _,
                    eol_colon: _,
                    stmts: _,
                } => todo!(),
                SemStmtData::IfElse {
                    if_branch: sem_if_branch,
                    elif_branches: sem_elif_branches,
                    else_branch: sem_else_branch,
                } => {
                    subtraces.push(
                        Trace::new_lazy_stmt(
                            parent_trace_path,
                            parent_trace,
                            LazyStmtEssence::IfBranch,
                            &mut registry,
                            stmt,
                            LazyStmtSketch::IfBranch {
                                if_regional_token: sem_if_branch.if_token(),
                                eol_colon_regional_token: sem_if_branch.eol_colon_token(),
                                stmts: sem_if_branch.stmts(),
                            },
                            sem_expr_region,
                            db,
                        )
                        .into(),
                    );
                    for (elif_branch_idx, sem_elif_branch) in sem_elif_branches.iter().enumerate() {
                        let elif_branch_idx = elif_branch_idx.try_into().unwrap();
                        subtraces.push(
                            Trace::new_lazy_stmt(
                                parent_trace_path,
                                parent_trace,
                                LazyStmtEssence::ElifBranch { elif_branch_idx },
                                &mut registry,
                                stmt,
                                LazyStmtSketch::ElifBranch {
                                    elif_branch_idx,
                                    elif_regional_token: sem_elif_branch.elif_regional_token(),
                                    eol_colon_regional_token: sem_elif_branch.eol_colon_token(),
                                    stmts: sem_elif_branch.stmts(),
                                },
                                sem_expr_region,
                                db,
                            )
                            .into(),
                        );
                    }
                    if let Some(sem_else_branch) = sem_else_branch {
                        subtraces.push(
                            Trace::new_lazy_stmt(
                                parent_trace_path,
                                parent_trace,
                                LazyStmtEssence::ElseBranch,
                                &mut registry,
                                stmt,
                                LazyStmtSketch::ElseBranch {
                                    else_regional_token: sem_else_branch.else_regional_token(),
                                    eol_colon_regional_token: sem_else_branch
                                        .eol_colon_regional_token(),
                                    stmts: sem_else_branch.stmts(),
                                },
                                sem_expr_region,
                                db,
                            )
                            .into(),
                        );
                    }
                }
                SemStmtData::Match { .. } => todo!(),
            }
        }
        subtraces
    }
}
