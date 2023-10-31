use husky_entity_path::PrincipalEntityPath;
use husky_hir_eager_expr::{
    builder::hir_eager_expr_region_with_source_map, HirEagerExprRegion, HirEagerStmtIdx,
};
use husky_regional_token::{
    ElifRegionalToken, ElseRegionalToken, EolColonRegionalToken, EolRegionalToken, IfRegionalToken,
    RegionalTokenIdxRange, StmtForRegionalToken,
};
use husky_sema_expr::{
    helpers::range::sema_expr_range_region, SemaExprRegion, SemaStmtData, SemaStmtIdx,
    SemaStmtIdxRange,
};
use husky_token_info::TokenInfoSource;

use crate::registry::associated_trace::IsAssociatedTraceRegistry;

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerStmtTracePath {
    pub parent_path: EagerStmtTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerStmtTracePathData,
    pub disambiguator: TracePathDisambiguator<EagerStmtTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerStmtTraceBiologicalParentPath {
    ValItem(ValItemTracePath),
    EagerCall(EagerCallTracePath),
    EagerStmt(EagerStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EagerStmtTracePathData {
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

impl EagerStmtTracePath {
    pub fn new(
        biological_parent_path: impl Into<EagerStmtTraceBiologicalParentPath>,
        path_data: EagerStmtTracePathData,
        registry: &mut TracePathRegistry<EagerStmtTracePathData>,
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
pub struct EagerStmtTrace {
    #[id]
    pub path: EagerStmtTracePath,
    pub biological_parent: EagerStmtTraceBiologicalParent,
    pub sema_stmt_idx: SemaStmtIdx,
    pub hir_eager_stmt_idx: Option<HirEagerStmtIdx>,
    pub data: EagerStmtTraceData,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerStmtTraceBiologicalParent {
    ValItem(ValItemTrace),
    EagerCall(EagerCallTrace),
    EagerStmt(EagerStmtTrace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EagerStmtTraceData {
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

impl EagerStmtTrace {
    pub fn new(
        biological_parent: impl Into<EagerStmtTraceBiologicalParent>,
        biological_parent_path: impl Into<EagerStmtTraceBiologicalParentPath>,
        trace_path_data: EagerStmtTracePathData,
        registry: &mut crate::registry::trace_path::TracePathRegistry<EagerStmtTracePathData>,
        sema_stmt_idx: SemaStmtIdx,
        trace_data: EagerStmtTraceData,
        sema_expr_region: SemaExprRegion,
        db: &dyn TraceDb,
    ) -> Self {
        let path = EagerStmtTracePath::new(biological_parent_path, trace_path_data, registry, db);
        let (hir_eager_expr_region, hir_eager_expr_source_map) =
            hir_eager_expr_region_with_source_map(db, sema_expr_region);
        let hir_eager_stmt_idx = hir_eager_expr_source_map
            .data(db)
            .sema_to_hir_eager_stmt_idx(sema_stmt_idx);
        EagerStmtTrace::new_inner(
            db,
            path,
            biological_parent.into(),
            sema_stmt_idx,
            hir_eager_stmt_idx,
            trace_data,
            sema_expr_region,
            hir_eager_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = eager_stmt_trace_view_lines(db, self);
        let have_subtraces = match self.data(db) {
            EagerStmtTraceData::BasicStmt => false,
            EagerStmtTraceData::IfBranch { .. } => true,
            EagerStmtTraceData::ElifBranch { .. } => true,
            EagerStmtTraceData::ElseBranch { .. } => true,
            EagerStmtTraceData::ForBetween { .. } => true,
            EagerStmtTraceData::ForIn { .. } => true,
        };
        TraceViewData::new(tokens.data().to_vec(), have_subtraces)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        eager_stmt_trace_subtraces(db, self)
    }

    pub(crate) fn from_stmts(
        parent_trace_path: impl Into<EagerStmtTraceBiologicalParentPath>,
        parent_trace: impl Into<EagerStmtTraceBiologicalParent>,
        stmts: husky_sema_expr::SemaStmtIdxRange,
        sema_expr_region: husky_sema_expr::SemaExprRegion,
        db: &dyn TraceDb,
    ) -> Vec<Trace> {
        let parent_trace = parent_trace.into();
        let parent_trace_path = parent_trace_path.into();
        let mut registry = TracePathRegistry::<EagerStmtTracePathData>::default();
        let mut subtraces: Vec<Trace> = vec![];
        let sema_stmt_arena = sema_expr_region.data(db).sema_stmt_arena();
        for stmt in stmts {
            match stmt.data(sema_stmt_arena) {
                SemaStmtData::Let {
                    let_token,
                    let_pattern_sema_obelisk,
                    eq_token,
                    initial_value_sema_expr_idx,
                } => {
                    let path_data = EagerStmtTracePathData::Let {};
                    let eager_stmt_trace = EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Return {
                    return_token,
                    result,
                } => {
                    let path_data = EagerStmtTracePathData::Return {};
                    let eager_stmt_trace = EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Require {
                    require_token,
                    condition,
                } => {
                    let path_data = EagerStmtTracePathData::Require {};
                    let eager_stmt_trace = EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Assert {
                    assert_token,
                    condition,
                } => {
                    let path_data = EagerStmtTracePathData::Assert {};
                    let eager_stmt_trace = EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Break { break_token } => {
                    let path_data = EagerStmtTracePathData::Break {};
                    let eager_stmt_trace = EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::BasicStmt,
                        sema_expr_region,
                        db,
                    );
                    subtraces.push(eager_stmt_trace.into())
                }
                SemaStmtData::Eval {
                    sema_expr_idx,
                    eol_semicolon,
                } => {
                    let path_data = EagerStmtTracePathData::Eval {};
                    let eager_stmt_trace = EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        path_data,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::BasicStmt,
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
                    EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        EagerStmtTracePathData::ForBetween,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::ForBetween {
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
                    condition,
                    eol_colon,
                    block,
                } => subtraces.push(
                    EagerStmtTrace::new(
                        parent_trace,
                        parent_trace_path,
                        EagerStmtTracePathData::ForIn,
                        &mut registry,
                        stmt,
                        EagerStmtTraceData::ForIn {
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
                    forext_token,
                    particulars,
                    eol_colon,
                    block,
                } => todo!(),
                SemaStmtData::While {
                    while_token,
                    condition,
                    eol_colon,
                    block,
                } => todo!(),
                SemaStmtData::DoWhile {
                    do_token,
                    while_token,
                    condition,
                    eol_colon,
                    block,
                } => todo!(),
                SemaStmtData::IfElse {
                    sema_if_branch,
                    sema_elif_branches,
                    sema_else_branch,
                } => {
                    subtraces.push(
                        EagerStmtTrace::new(
                            parent_trace,
                            parent_trace_path,
                            EagerStmtTracePathData::IfBranch,
                            &mut registry,
                            stmt,
                            EagerStmtTraceData::IfBranch {
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
                            EagerStmtTrace::new(
                                parent_trace,
                                parent_trace_path,
                                EagerStmtTracePathData::ElifBranch { elif_branch_idx },
                                &mut registry,
                                stmt,
                                EagerStmtTraceData::ElifBranch {
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
                            EagerStmtTrace::new(
                                parent_trace,
                                parent_trace_path,
                                EagerStmtTracePathData::ElseBranch,
                                &mut registry,
                                stmt,
                                EagerStmtTraceData::ElseBranch {
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
                    match_token,
                    match_target_sema_expr_idx,
                    eol_with_token,
                    sema_case_branches,
                } => todo!(),
            }
        }
        subtraces
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_stmt_trace_view_lines(db: &dyn TraceDb, trace: EagerStmtTrace) -> TraceViewLines {
    let sema_stmt_idx = trace.sema_stmt_idx(db);
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let region_path = sema_expr_region.path(db);
    let regional_token_idx_range = match trace.data(db) {
        EagerStmtTraceData::BasicStmt => sema_expr_range_region.data(db)[sema_stmt_idx],
        EagerStmtTraceData::IfBranch {
            if_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            if_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
        EagerStmtTraceData::ElifBranch {
            elif_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            elif_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
        EagerStmtTraceData::ElseBranch {
            else_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            else_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
        EagerStmtTraceData::ForBetween {
            for_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            for_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
        EagerStmtTraceData::ForIn {
            for_regional_token,
            eol_colon_regional_token,
            ..
        } => RegionalTokenIdxRange::new_closed(
            for_regional_token.regional_token_idx(),
            eol_colon_regional_token.regional_token_idx(),
        ),
    };
    let token_idx_range =
        regional_token_idx_range.token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    let registry = EagerStmtAssociatedTraceRegistry::new(trace, sema_expr_region);
    TraceViewLines::new(region_path.module_path(db), token_idx_range, registry, db)
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_stmt_trace_subtraces(db: &dyn TraceDb, trace: EagerStmtTrace) -> Vec<Trace> {
    match trace.data(db) {
        EagerStmtTraceData::BasicStmt => unreachable!(),
        EagerStmtTraceData::IfBranch { stmts, .. }
        | EagerStmtTraceData::ElifBranch { stmts, .. }
        | EagerStmtTraceData::ElseBranch { stmts, .. }
        | EagerStmtTraceData::ForIn { stmts, .. }
        | EagerStmtTraceData::ForBetween { stmts, .. } => {
            EagerStmtTrace::from_stmts(trace.path(db), trace, stmts, trace.sema_expr_region(db), db)
        }
    }
}

struct EagerStmtAssociatedTraceRegistry {
    parent_trace: EagerStmtTrace,
    sema_expr_region: SemaExprRegion,
    eager_expr_trace_path_registry: TracePathRegistry<EagerExprTracePathData>,
    sema_expr_traces_issued: VecPairMap<SemaExprIdx, EagerExprTrace>,
    syn_pattern_expr_traces_issued: VecPairMap<SynPatternExprIdx, EagerExprTrace>,
}

impl EagerStmtAssociatedTraceRegistry {
    fn new(parent_trace: EagerStmtTrace, sema_expr_region: SemaExprRegion) -> Self {
        EagerStmtAssociatedTraceRegistry {
            parent_trace,
            sema_expr_region,
            eager_expr_trace_path_registry: Default::default(),
            sema_expr_traces_issued: Default::default(),
            syn_pattern_expr_traces_issued: Default::default(),
        }
    }
}

impl IsAssociatedTraceRegistry for EagerStmtAssociatedTraceRegistry {
    fn get_or_issue_associated_trace(
        &mut self,
        source: TokenInfoSource,
        db: &dyn TraceDb,
    ) -> Option<Trace> {
        match source {
            TokenInfoSource::UseExpr(_) => None,
            TokenInfoSource::SemaExpr(sema_expr_idx) => Some(
                self.sema_expr_traces_issued
                    .get_value_copied_or_insert_with(sema_expr_idx, || {
                        EagerExprTrace::new(
                            self.parent_trace.path(db),
                            self.parent_trace,
                            sema_expr_idx,
                            self.sema_expr_region,
                            &mut self.eager_expr_trace_path_registry,
                            db,
                        )
                    })
                    .into(),
            ),
            TokenInfoSource::SynPrincipalEntityPathExpr(
                syn_principal_entity_path_expr_idx,
                syn_principal_entity_path,
            ) => match syn_principal_entity_path {
                PrincipalEntityPath::Module(_) => None,
                PrincipalEntityPath::MajorItem(major_item_path) => {
                    Trace::from_major_item_path(major_item_path, db)
                }
                PrincipalEntityPath::TypeVariant(_) => None,
            },
            TokenInfoSource::PatternExpr(syn_pattern_expr_idx) => Some(
                self.syn_pattern_expr_traces_issued
                    .get_value_copied_or_insert_with(syn_pattern_expr_idx, || {
                        EagerExprTrace::new(
                            self.parent_trace.path(db),
                            self.parent_trace,
                            syn_pattern_expr_idx,
                            self.sema_expr_region,
                            &mut self.eager_expr_trace_path_registry,
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
