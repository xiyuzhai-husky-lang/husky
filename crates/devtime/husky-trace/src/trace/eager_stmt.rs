use husky_entity_path::PrincipalEntityPath;
use husky_hir_eager_expr::{
    builder::hir_eager_expr_region_with_source_map,
    HirEagerExprRegion, HirEagerExprSourceMap, HirEagerExprSourceMapData, HirEagerStmtIdx,
};
use husky_regional_token::{
    ElifRegionalToken, ElseRegionalToken, EolColonRegionalToken, EolRegionalToken, IfRegionalToken,
    RegionalTokenIdxRange, StmtForRegionalToken,
};
use husky_sema_expr::{
    helpers::range::sema_expr_range_region, SemaExprData, SemaExprDb, SemaExprRegion, SemaStmtData,
    SemaStmtIdx, SemaStmtIdxRange,
};
use husky_syn_defn::ItemSynDefn;
use husky_token_info::TokenInfoSource;

use crate::registry::associated_trace::IsAssociatedTraceRegistry;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EagerStmtTracePathData {
    biological_parent_path: TracePath,
    essence: EagerStmtEssence,
    disambiguator: TracePathDisambiguator<EagerStmtEssence>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerStmtEssence {
    Let,
    Return,
    Require,
    Assert,
    Break,
    Eval,
    IfBranch,
    ElifBranch { elif_branch_idx: u8 },
    ElseBranch,
    ForIn,
    ForBetween,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerStmtTraceData {
    pub path: TracePath,
    pub biological_parent: Trace,
    pub sema_stmt_idx: SemaStmtIdx,
    pub hir_eager_stmt_idx: Option<HirEagerStmtIdx>,
    pub eager_stmt_data_sketch: EagerStmtDataSketch,
    // #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    // #[skip_fmt]
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EagerStmtDataSketch {
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
    ForIn {
        for_regional_token: StmtForRegionalToken,
        eol_colon_regional_token: EolRegionalToken,
        stmts: SemaStmtIdxRange,
    },
    ForBetween {
        for_regional_token: StmtForRegionalToken,
        eol_colon_regional_token: EolRegionalToken,
        stmts: SemaStmtIdxRange,
    },
}

impl Trace {
    pub(crate) fn new_eager_stmt(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        essence: EagerStmtEssence,
        registry: &mut crate::registry::trace_path::TracePathRegistry<EagerStmtEssence>,
        sema_stmt_idx: SemaStmtIdx,
        eager_stmt_data_sketch: EagerStmtDataSketch,
        sema_expr_region: SemaExprRegion,
        db: &::salsa::Db,
    ) -> Self {
        let path = TracePath::new(
            EagerStmtTracePathData {
                biological_parent_path: biological_parent_path.into(),
                essence,
                disambiguator: registry.issue(essence),
            },
            db,
        );
        let (hir_eager_expr_region, hir_eager_expr_source_map) =
            hir_eager_expr_region_with_source_map(db, sema_expr_region);
        let hir_eager_stmt_idx = hir_eager_expr_source_map
            .data(db)
            .sema_to_hir_eager_stmt_idx(sema_stmt_idx);
        Trace::new(
            path,
            EagerStmtTraceData {
                path,
                biological_parent: biological_parent.into(),
                sema_stmt_idx,
                hir_eager_stmt_idx,
                eager_stmt_data_sketch,
                sema_expr_region,
                hir_eager_expr_region,
            }
            .into(),
            db,
        )
    }

    pub(crate) fn new_eager_stmts_from_syn_body_with_syn_expr_region(
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
        let sema_expr_region = db.sema_expr_region(syn_expr_region);
        let sema_expr_region_data = sema_expr_region.data(db);
        let body = sema_expr_region_data.syn_root_to_sema_expr_idx(body);
        let SemaExprData::Block { stmts } = *body.data(sema_expr_region_data.sema_expr_arena())
        else {
            unreachable!()
        };
        Self::new_eager_stmts(parent_trace_path, parent_trace, stmts, sema_expr_region, db)
    }

    pub(crate) fn new_eager_stmts(
        parent_trace_path: TracePath,
        parent_trace: Trace,
        stmts: husky_sema_expr::SemaStmtIdxRange,
        sema_expr_region: husky_sema_expr::SemaExprRegion,
        db: &::salsa::Db,
    ) -> Vec<Trace> {
        let mut registry = TracePathRegistry::<EagerStmtEssence>::default();
        let mut subtraces: Vec<Trace> = vec![];
        let sema_stmt_arena = sema_expr_region.data(db).sema_stmt_arena();
        for stmt in stmts {
            match stmt.data(sema_stmt_arena) {
                SemaStmtData::Let { .. } => {
                    let essence = EagerStmtEssence::Let {};
                    let eager_stmt_trace = Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        essence,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Return { .. } => {
                    let essence = EagerStmtEssence::Return {};
                    let eager_stmt_trace = Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        essence,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Require {
                    require_token: _,
                    condition: _,
                } => {
                    let path_data = EagerStmtEssence::Require {};
                    let eager_stmt_trace = Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Assert {
                    assert_token: _,
                    condition: _,
                } => {
                    let path_data = EagerStmtEssence::Assert {};
                    let eager_stmt_trace = Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Break { break_token: _ } => {
                    let path_data = EagerStmtEssence::Break {};
                    let eager_stmt_trace = Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Eval { .. } => {
                    let path_data = EagerStmtEssence::Eval {};
                    let eager_stmt_trace = Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                &SemaStmtData::ForBetween {
                    for_token,
                    eol_colon,
                    block,
                    ..
                } => subtraces.push(
                    Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        EagerStmtEssence::ForBetween,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::ForBetween {
                            for_regional_token: for_token,
                            eol_colon_regional_token: eol_colon,
                            stmts: block,
                        },
                        sema_expr_region,
                        db,
                    )
                    .into(),
                ),
                &SemaStmtData::ForIn {
                    for_token,
                    condition: _,
                    eol_colon,
                    block,
                } => subtraces.push(
                    Trace::new_eager_stmt(
                        parent_trace_path,
                        parent_trace,
                        EagerStmtEssence::ForIn,
                        &mut registry,
                        stmt,
                        EagerStmtDataSketch::ForIn {
                            for_regional_token: for_token,
                            eol_colon_regional_token: eol_colon,
                            stmts: block,
                        },
                        sema_expr_region,
                        db,
                    )
                    .into(),
                ),
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
                    if_branch: sema_if_branch,
                    elif_branches: sema_elif_branches,
                    else_branch: sema_else_branch,
                } => {
                    subtraces.push(
                        Trace::new_eager_stmt(
                            parent_trace_path,
                            parent_trace,
                            EagerStmtEssence::IfBranch,
                            &mut registry,
                            stmt,
                            EagerStmtDataSketch::IfBranch {
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
                            Trace::new_eager_stmt(
                                parent_trace_path,
                                parent_trace,
                                EagerStmtEssence::ElifBranch { elif_branch_idx },
                                &mut registry,
                                stmt,
                                EagerStmtDataSketch::ElifBranch {
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
                            Trace::new_eager_stmt(
                                parent_trace_path,
                                parent_trace,
                                EagerStmtEssence::ElseBranch,
                                &mut registry,
                                stmt,
                                EagerStmtDataSketch::ElseBranch {
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

impl EagerStmtTraceData {
    pub(super) fn view_lines(&self, trace_id: Trace, db: &::salsa::Db) -> TraceViewLines {
        let sema_stmt_idx = self.sema_stmt_idx;
        let sema_expr_region = self.sema_expr_region;
        let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
        let region_path = sema_expr_region.path(db);
        let regional_token_idx_range = match self.eager_stmt_data_sketch {
            EagerStmtDataSketch::BasicStmt => sema_expr_range_region.data(db)[sema_stmt_idx],
            EagerStmtDataSketch::IfBranch {
                if_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                if_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
            EagerStmtDataSketch::ElifBranch {
                elif_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                elif_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
            EagerStmtDataSketch::ElseBranch {
                else_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                else_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
            EagerStmtDataSketch::ForBetween {
                for_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                for_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
            EagerStmtDataSketch::ForIn {
                for_regional_token,
                eol_colon_regional_token,
                ..
            } => RegionalTokenIdxRange::new_closed(
                for_regional_token.regional_token_idx(),
                eol_colon_regional_token.regional_token_idx(),
            ),
        };
        let token_idx_range = regional_token_idx_range
            .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
        let registry = EagerStmtAssociatedTraceRegistry::new(trace_id, sema_expr_region, db);
        TraceViewLines::new(region_path.module_path(db), token_idx_range, registry, db)
    }

    pub(super) fn have_subtraces(&self, _db: &::salsa::Db) -> bool {
        match self.eager_stmt_data_sketch {
            EagerStmtDataSketch::BasicStmt => false,
            EagerStmtDataSketch::IfBranch { .. } => true,
            EagerStmtDataSketch::ElifBranch { .. } => true,
            EagerStmtDataSketch::ElseBranch { .. } => true,
            EagerStmtDataSketch::ForBetween { .. } => true,
            EagerStmtDataSketch::ForIn { .. } => true,
        }
    }

    pub(super) fn subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        match self.eager_stmt_data_sketch {
            EagerStmtDataSketch::BasicStmt => vec![],
            EagerStmtDataSketch::IfBranch { stmts, .. }
            | EagerStmtDataSketch::ElifBranch { stmts, .. }
            | EagerStmtDataSketch::ElseBranch { stmts, .. }
            | EagerStmtDataSketch::ForIn { stmts, .. }
            | EagerStmtDataSketch::ForBetween { stmts, .. } => {
                Trace::new_eager_stmts(self.path, trace, stmts, self.sema_expr_region, db)
            }
        }
    }
}

struct EagerStmtAssociatedTraceRegistry<'a> {
    parent_trace: Trace,
    sema_expr_region: SemaExprRegion,
    hir_eager_expr_region: HirEagerExprRegion,
    syn_expr_region_data: &'a SynExprRegionData,
    hir_eager_expr_source_map: HirEagerExprSourceMap,
    hir_eager_expr_source_map_data: &'a HirEagerExprSourceMapData,
    eager_expr_trace_path_registry: TracePathRegistry<EagerExprEssence>,
    eager_expr_traces_issued: VecPairMap<SemaExprIdx, Trace>,
    eager_pattern_expr_trace_path_registry: TracePathRegistry<EagerPatternExprEssence>,
    eager_pattern_expr_traces_issued: VecPairMap<SynPatternExprIdx, Trace>,
}

impl<'a> EagerStmtAssociatedTraceRegistry<'a> {
    fn new(parent_trace: Trace, sema_expr_region: SemaExprRegion, db: &'a ::salsa::Db) -> Self {
        let (hir_eager_expr_region, hir_eager_expr_source_map) =
            hir_eager_expr_region_with_source_map(db, sema_expr_region);
        EagerStmtAssociatedTraceRegistry {
            parent_trace,
            sema_expr_region,
            syn_expr_region_data: sema_expr_region.syn_expr_region(db).data(db),
            hir_eager_expr_region,
            hir_eager_expr_source_map,
            hir_eager_expr_source_map_data: hir_eager_expr_source_map.data(db),
            eager_expr_trace_path_registry: Default::default(),
            eager_expr_traces_issued: Default::default(),
            eager_pattern_expr_trace_path_registry: Default::default(),
            eager_pattern_expr_traces_issued: Default::default(),
        }
    }
}

impl<'a> IsAssociatedTraceRegistry for EagerStmtAssociatedTraceRegistry<'a> {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &::salsa::Db,
    ) -> Option<Trace> {
        match source {
            TokenInfoSource::UseExpr(_) => None,
            TokenInfoSource::SemaExpr(sema_expr_idx) => Some(
                self.eager_expr_traces_issued
                    .get_value_copied_or_insert_with(sema_expr_idx, || {
                        let hir_eager_expr_idx = self
                            .hir_eager_expr_source_map_data
                            .sema_to_hir_eager_expr_idx(sema_expr_idx);
                        Trace::new_eager_expr(
                            self.parent_trace.path(db),
                            self.parent_trace,
                            sema_expr_idx,
                            hir_eager_expr_idx,
                            self.sema_expr_region,
                            self.hir_eager_expr_region,
                            self.hir_eager_expr_source_map,
                            &mut self.eager_expr_trace_path_registry,
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
                self.eager_pattern_expr_traces_issued
                    .get_value_copied_or_insert_with(syn_pattern_expr_idx, || {
                        Trace::new_eager_pattern_expr(
                            self.parent_trace.path(db),
                            self.parent_trace,
                            syn_pattern_expr_idx,
                            self.syn_expr_region_data
                                .syn_pattern_expr_current_syn_symbols_mapped(
                                    syn_pattern_expr_idx,
                                    |current_syn_symbol_idx| {
                                        self.hir_eager_expr_source_map_data
                                            .current_syn_symbol_to_hir_eager_runtime_symbol(
                                                current_syn_symbol_idx,
                                            )
                                    },
                                ),
                            self.sema_expr_region,
                            &mut self.eager_pattern_expr_trace_path_registry,
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
