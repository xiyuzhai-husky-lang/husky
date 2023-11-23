use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_hir_lazy_expr::{
    source_map::{HirLazyExprSourceMap, HirLazyExprSourceMapData},
    HirLazyExprData, HirLazyExprIdx, HirLazyExprRegion,
};
use husky_sema_expr::{
    helpers::range::sema_expr_range_region, SemaExprData, SemaExprRegion,
    SemaRitchieParameterArgumentMatch,
};
use husky_val_repr::expansion::ValReprExpansion;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyExprTracePath {
    pub biological_parent_path: LazyExprTraceBiologicalParentPath,
    #[return_ref]
    pub data: LazyExprTracePathData,
    pub disambiguator: TracePathDisambiguator<LazyExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum LazyExprTraceBiologicalParentPath {
    LazyStmt(LazyStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LazyExprTracePathData {
    AdHoc,
}

impl LazyExprTracePath {
    fn new(
        biological_parent_path: LazyExprTraceBiologicalParentPath,
        path_data: LazyExprTracePathData,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            biological_parent_path,
            path_data.clone(),
            lazy_expr_trace_path_registry.issue(path_data),
        )
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct LazyExprTrace {
    #[id]
    pub path: LazyExprTracePath,
    pub biological_parent: LazyExprTraceBiologicalParent,
    pub sema_expr_idx: SemaExprIdx,
    pub hir_lazy_expr_idx: Option<HirLazyExprIdx>,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_lazy_expr_region: HirLazyExprRegion,
    #[skip_fmt]
    pub hir_lazy_expr_source_map: HirLazyExprSourceMap,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum LazyExprTraceBiologicalParent {
    LazyStmt(LazyStmtTrace),
}

impl LazyExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<LazyExprTraceBiologicalParentPath>,
        biological_parent: impl Into<LazyExprTraceBiologicalParent>,
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr_idx: Option<HirLazyExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_lazy_expr_region: HirLazyExprRegion,
        hir_lazy_expr_source_map: HirLazyExprSourceMap,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = LazyExprTracePathData::AdHoc;
        let path = LazyExprTracePath::new(
            biological_parent_path.into(),
            path_data,
            lazy_expr_trace_path_registry,
            db,
        );
        Self::new_inner(
            db,
            path,
            biological_parent.into(),
            sema_expr_idx,
            hir_lazy_expr_idx,
            sema_expr_region,
            hir_lazy_expr_region,
            hir_lazy_expr_source_map,
        )
    }

    pub fn view_lines<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewLines {
        lazy_expr_trace_view_lines(db, self)
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        lazy_expr_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        lazy_expr_trace_subtraces(db, self)
    }

    pub fn val_repr(self, db: &dyn TraceDb) -> Option<ValRepr> {
        lazy_expr_trace_val_repr(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_expr_trace_view_lines(db: &dyn TraceDb, trace: LazyExprTrace) -> TraceViewLines {
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let sema_expr_range_region_data = sema_expr_range_region.data(db);
    let region_path = sema_expr_region.path(db);
    let regional_token_idx_range = sema_expr_range_region_data[trace.sema_expr_idx(db)];
    let token_idx_range =
        regional_token_idx_range.token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    TraceViewLines::new(
        region_path.module_path(db),
        token_idx_range,
        VoidAssociatedTraceRegistry,
        db,
    )
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_expr_trace_have_subtraces(db: &dyn TraceDb, trace: LazyExprTrace) -> bool {
    use husky_hir_defn::defn::HasHirDefn;
    let Some(hir_eager_expr_idx) = trace.hir_lazy_expr_idx(db) else {
        return false;
    };
    match trace.hir_lazy_expr_region(db).hir_lazy_expr_arena(db)[hir_eager_expr_idx] {
        HirLazyExprData::FunctionFnItemCall { path, .. } => path.hir_defn(db).is_some(),
        HirLazyExprData::AssociatedFunctionFnCall { path, .. } => path.hir_defn(db).is_some(),
        HirLazyExprData::MethodFnCall { path, .. } => path.hir_defn(db).is_some(),
        HirLazyExprData::AssociatedFn { path } => path.hir_defn(db).is_some(),
        HirLazyExprData::FunctionGnItemCall { path, .. } => path.hir_defn(db).is_some(),
        HirLazyExprData::Block { stmts } => unreachable!(),
        _ => false,
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn lazy_expr_trace_subtraces(db: &dyn TraceDb, trace: LazyExprTrace) -> Vec<Trace> {
    let sema_expr_idx = trace.sema_expr_idx(db);
    let Some(hir_eager_expr_idx) = trace.hir_lazy_expr_idx(db) else {
        return vec![];
    };
    let sema_expr_region_data = trace.sema_expr_region(db).data(db);
    let hir_lazy_expr_source_map_data = trace.hir_lazy_expr_source_map(db).data(db);
    match trace.hir_lazy_expr_region(db).hir_lazy_expr_arena(db)[hir_eager_expr_idx] {
        HirLazyExprData::FunctionFnItemCall { path, .. } => {
            let SemaExprData::FunctionFnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let trace_path = trace.path(db);
            let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                hir_lazy_expr_source_map_data,
                db,
            );
            subtraces.push(LazyCallTrace::new(trace_path, trace, path.into(), db).into());
            subtraces
        }
        HirLazyExprData::AssociatedFunctionFnCall { path, .. } => {
            let SemaExprData::FunctionFnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let trace_path = trace.path(db);
            let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                hir_lazy_expr_source_map_data,
                db,
            );
            subtraces.push(LazyCallTrace::new(trace_path, trace, path.into(), db).into());
            subtraces
        }
        HirLazyExprData::MethodFnCall { path, .. } => {
            let SemaExprData::FunctionFnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let trace_path = trace.path(db);
            let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                hir_lazy_expr_source_map_data,
                db,
            );
            subtraces.push(LazyCallTrace::new(trace_path, trace, path.into(), db).into());
            subtraces
        }
        HirLazyExprData::Block { .. } => unreachable!(),
        HirLazyExprData::FunctionGnItemCall { path, .. } => {
            let SemaExprData::FunctionGnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let trace_path = trace.path(db);
            let mut subtraces: Vec<Trace> = fn_call_lazy_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                hir_lazy_expr_source_map_data,
                db,
            );
            subtraces.push(LazyCallTrace::new(trace_path, trace, path.into(), db).into());
            subtraces
        }
        _ => vec![],
    }
}

fn fn_call_lazy_expr_trace_input_traces(
    trace_path: LazyExprTracePath,
    trace: LazyExprTrace,
    ritchie_parameter_argument_matches: &[SemaRitchieParameterArgumentMatch],
    hir_lazy_expr_source_map_data: &HirLazyExprSourceMapData,
    db: &dyn TraceDb,
) -> Vec<Trace> {
    ritchie_parameter_argument_matches
        .iter()
        .map(|m| {
            let data = match m {
                SemaRitchieParameterArgumentMatch::Regular(_, list_item) => {
                    let sema_expr_idx = list_item.argument_sema_expr_idx();
                    LazyCallInputTraceData::Regular {
                        sema_expr_idx,
                        hir_lazy_expr_idx: hir_lazy_expr_source_map_data
                            .sema_to_hir_lazy_expr_idx(sema_expr_idx),
                    }
                }
                SemaRitchieParameterArgumentMatch::Variadic(_, _) => {
                    todo!()
                }
                SemaRitchieParameterArgumentMatch::Keyed(_, _) => {
                    todo!()
                }
            };
            LazyCallInputTrace::new(trace_path, trace, data, db).into()
        })
        .collect()
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_expr_trace_val_repr(db: &dyn TraceDb, trace: LazyExprTrace) -> Option<ValRepr> {
    let val_repr_expansion = lazy_expr_trace_val_repr_expansion(db, trace);
    val_repr_expansion
        .hir_lazy_expr_val_repr_map(db)
        .get(trace.hir_lazy_expr_idx(db)?)
        .copied()
}

#[salsa::tracked(jar = TraceJar)]
fn lazy_expr_trace_val_repr_expansion(db: &dyn TraceDb, trace: LazyExprTrace) -> ValReprExpansion {
    match trace.biological_parent(db) {
        LazyExprTraceBiologicalParent::LazyStmt(trace) => {
            lazy_stmt_trace_val_repr_expansion(db, trace)
        }
    }
}
