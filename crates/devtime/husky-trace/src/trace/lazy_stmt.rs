use super::*;
use crate::registry::{
    associated_trace::IsAssociatedTraceRegistry,
    trace_path::{TracePathDisambiguator, TracePathRegistry},
};
use husky_entity_path::PrincipalEntityPath;
use husky_hir_lazy_expr::HirLazyStmtIdx;
use husky_hir_lazy_expr::{builder::hir_lazy_expr_region_with_source_map, HirLazyExprRegion};
use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprRegion, SemaStmtIdx};
use husky_token_info::TokenInfoSource;

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
    pub fn new(
        biological_parent_path: impl Into<LazyStmtTraceBiologicalParentPath>,
        path_data: LazyStmtTracePathData,
        registry: &mut TracePathRegistry<LazyStmtTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        LazyStmtTracePath::new_inner(
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
    IfBranch,
    ElifBranch { elif_branch_idx: u8 },
    ElseBranch,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyStmtTraceBiologicalParent {
    ValItem(ValItemTrace),
    LazyCall(LazyCallTrace),
    LazyStmt(LazyStmtTrace),
}

impl LazyStmtTrace {
    pub fn new(
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
        let tokens = lazy_stmt_trace_view_tokens(db, self);
        let have_subtraces = match self.data(db) {
            LazyStmtTraceData::BasicStmt => false,
            LazyStmtTraceData::IfBranch => true,
            LazyStmtTraceData::ElifBranch { elif_branch_idx } => true,
            LazyStmtTraceData::ElseBranch => true,
        };
        TraceViewData::new(tokens.data().to_vec(), have_subtraces)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        match self.data(db) {
            LazyStmtTraceData::BasicStmt => unreachable!("shouldn't be here"),
            LazyStmtTraceData::IfBranch => todo!(),
            LazyStmtTraceData::ElifBranch { elif_branch_idx } => todo!(),
            LazyStmtTraceData::ElseBranch => todo!(),
        }
    }

    pub fn associated_expr_traces<'a>(self, db: &'a dyn TraceDb) -> &'a [(SemaExprIdx, Trace)] {
        lazy_stmt_associated_expr_traces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_trace_view_tokens(db: &dyn TraceDb, trace: LazyStmtTrace) -> TraceViewTokens {
    let sema_stmt_idx = trace.sema_stmt_idx(db);
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let region_path = sema_expr_region.path(db);
    let token_idx_range = sema_expr_range_region.data(db)[sema_stmt_idx]
        .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    let registry = LazyStmtAssociatedTraceRegistry::new(trace, sema_expr_region);
    TraceViewTokens::new(region_path.module_path(db), token_idx_range, registry, db)
}

struct LazyStmtAssociatedTraceRegistry {
    parent_trace: LazyStmtTrace,
    sema_expr_region: SemaExprRegion,
    lazy_expr_trace_path_registry: TracePathRegistry<LazyExprTracePathData>,
    sema_expr_traces_issued: VecPairMap<SemaExprIdx, LazyExprTrace>,
    syn_pattern_expr_traces_issued: VecPairMap<SynPatternExprIdx, LazyExprTrace>,
}

impl LazyStmtAssociatedTraceRegistry {
    fn new(parent_trace: LazyStmtTrace, sema_expr_region: SemaExprRegion) -> Self {
        LazyStmtAssociatedTraceRegistry {
            parent_trace,
            sema_expr_region,
            lazy_expr_trace_path_registry: Default::default(),
            sema_expr_traces_issued: Default::default(),
            syn_pattern_expr_traces_issued: Default::default(),
        }
    }
}

impl IsAssociatedTraceRegistry for LazyStmtAssociatedTraceRegistry {
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
                        LazyExprTrace::new(
                            self.parent_trace.path(db),
                            self.parent_trace,
                            sema_expr_idx,
                            self.sema_expr_region,
                            &mut self.lazy_expr_trace_path_registry,
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
                        LazyExprTrace::new(
                            self.parent_trace.path(db),
                            self.parent_trace,
                            syn_pattern_expr_idx,
                            self.sema_expr_region,
                            &mut self.lazy_expr_trace_path_registry,
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

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_stmt_associated_expr_traces(
    db: &dyn TraceDb,
    trace: LazyStmtTrace,
) -> VecPairMap<SemaExprIdx, Trace> {
    todo!()
}
