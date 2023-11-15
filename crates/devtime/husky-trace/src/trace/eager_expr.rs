use super::*;
use crate::registry::associated_trace::VoidAssociatedTraceRegistry;
use husky_hir_decl::FugitiveHirDecl;
use husky_hir_defn::HasHirDefn;
use husky_hir_eager_expr::{
    HirEagerExprData, HirEagerExprIdx, HirEagerExprRegion, HirEagerExprSourceMap,
    HirEagerExprSourceMapData,
};
use husky_sema_expr::{
    helpers::range::sema_expr_range_region, SemaExprData, SemaExprRegion,
    SemaRitchieParameterArgumentMatch,
};

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerExprTracePath {
    pub biological_parent_path: EagerExprTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerExprTracePathData,
    pub disambiguator: TracePathDisambiguator<EagerExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParentPath {
    EagerStmt(EagerStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerExprTracePathData {
    Haha,
}

impl EagerExprTracePath {
    fn new(
        biological_parent_path: EagerExprTraceBiologicalParentPath,
        path_data: EagerExprTracePathData,
        eager_expr_trace_path_registry: &mut TracePathRegistry<EagerExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            biological_parent_path,
            path_data.clone(),
            eager_expr_trace_path_registry.issue(path_data),
        )
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerExprTrace {
    #[id]
    pub path: EagerExprTracePath,
    pub biological_parent: EagerExprTraceBiologicalParent,
    pub sema_expr_idx: SemaExprIdx,
    pub hir_eager_expr_idx: Option<HirEagerExprIdx>,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
    #[skip_fmt]
    pub hir_eager_expr_region: HirEagerExprRegion,
    #[skip_fmt]
    pub hir_eager_expr_source_map: HirEagerExprSourceMap,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

impl EagerExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerExprTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerExprTraceBiologicalParent>,
        sema_expr_idx: SemaExprIdx,
        hir_eager_expr_idx: Option<HirEagerExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_eager_expr_region: HirEagerExprRegion,
        hir_eager_expr_source_map: HirEagerExprSourceMap,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<EagerExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = EagerExprTracePathData::Haha;
        let path = EagerExprTracePath::new(
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
            hir_eager_expr_idx,
            sema_expr_region,
            hir_eager_expr_region,
            hir_eager_expr_source_map,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let trace_view_lines = eager_expr_trace_view_lines(db, self);
        TraceViewData::new(trace_view_lines.data().to_vec(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        eager_expr_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        eager_expr_trace_subtraces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_expr_trace_view_lines(db: &dyn TraceDb, trace: EagerExprTrace) -> TraceViewLines {
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
fn eager_expr_trace_have_subtraces(db: &dyn TraceDb, trace: EagerExprTrace) -> bool {
    use husky_hir_defn::defn::HasHirDefn;
    let Some(hir_eager_expr_idx) = trace.hir_eager_expr_idx(db) else {
        return false;
    };
    match trace.hir_eager_expr_region(db).hir_eager_expr_arena(db)[hir_eager_expr_idx] {
        HirEagerExprData::FunctionFnCall { path, .. } => path.hir_defn(db).is_some(),
        HirEagerExprData::AssociatedFunctionFnCall { path, .. } => path.hir_defn(db).is_some(),
        HirEagerExprData::MethodFnCall { path, .. } => path.hir_defn(db).is_some(),
        HirEagerExprData::Block { stmts } => unreachable!(),
        HirEagerExprData::AssociatedFn {
            associated_item_path,
        } => associated_item_path.hir_defn(db).is_some(),
        _ => false,
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_expr_trace_subtraces(db: &dyn TraceDb, trace: EagerExprTrace) -> Vec<Trace> {
    use husky_syn_defn::HasSynDefn;
    let sema_expr_idx = trace.sema_expr_idx(db);
    let Some(hir_eager_expr_idx) = trace.hir_eager_expr_idx(db) else {
        return vec![];
    };
    let caller_sema_expr_region = trace.sema_expr_region(db);
    let caller_sema_expr_region_data = caller_sema_expr_region.data(db);
    let hir_eager_expr_source_map_data = trace.hir_eager_expr_source_map(db).data(db);
    match trace.hir_eager_expr_region(db).hir_eager_expr_arena(db)[hir_eager_expr_idx] {
        HirEagerExprData::FunctionFnCall {
            path,
            ref item_groups,
            ..
        } => {
            let SemaExprData::FunctionFnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(caller_sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let Some(hir_defn) = path.hir_defn(db) else {
                return vec![];
            };
            let FugitiveHirDecl::FunctionFn(hir_decl) = hir_defn.hir_decl(db) else {
                unreachable!()
            };
            let trace_path = trace.path(db);
            let mut subtraces = fn_call_eager_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                caller_sema_expr_region,
                hir_eager_expr_source_map_data,
                db,
            );
            subtraces.push(
                EagerCallTrace::new(
                    trace_path,
                    trace,
                    EagerCallTraceData::FunctionFn { path },
                    db,
                )
                .into(),
            );
            subtraces
        }
        HirEagerExprData::AssociatedFunctionFnCall {
            path,
            ref item_groups,
            ..
        } => {
            let SemaExprData::FunctionFnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(caller_sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let syn_defn = path.syn_defn(db).unwrap();
            let syn_decl = syn_defn.decl(db) else {
                unreachable!()
            };
            hir_decl.parenate_parameters(db);
            let trace_path = trace.path(db);
            let mut subtraces = fn_call_eager_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                caller_sema_expr_region,
                hir_eager_expr_source_map_data,
                db,
            );
            subtraces.push(
                EagerCallTrace::new(
                    trace_path,
                    trace,
                    EagerCallTraceData::AssociatedFunctionFn { path },
                    db,
                )
                .into(),
            );
            subtraces
        }
        HirEagerExprData::MethodFnCall {
            path,
            ref item_groups,
            ..
        } => {
            let SemaExprData::MethodFnCall {
                ref ritchie_parameter_argument_matches,
                ..
            } = sema_expr_idx.data(caller_sema_expr_region_data.sema_expr_arena())
            else {
                unreachable!()
            };
            let Some(hir_defn) = path.hir_defn(db) else {
                return vec![];
            };
            let trace_path = trace.path(db);
            let mut subtraces = fn_call_eager_expr_trace_input_traces(
                trace_path,
                trace,
                ritchie_parameter_argument_matches,
                caller_sema_expr_region,
                hir_eager_expr_source_map_data,
                db,
            );
            subtraces.push(
                EagerCallTrace::new(trace_path, trace, EagerCallTraceData::MethodFn { path }, db)
                    .into(),
            );
            subtraces
        }
        HirEagerExprData::Block { .. } => unreachable!(),
        HirEagerExprData::AssociatedFn {
            associated_item_path,
        } => todo!(),
        _ => vec![],
    }
}

fn fn_call_eager_expr_trace_input_traces(
    trace_path: EagerExprTracePath,
    trace: EagerExprTrace,
    ritchie_parameter_argument_matches: &[SemaRitchieParameterArgumentMatch],
    caller_sema_expr_region: SemaExprRegion,
    caller_hir_eager_expr_source_map_data: &HirEagerExprSourceMapData,
    db: &dyn TraceDb,
) -> Vec<Trace> {
    ritchie_parameter_argument_matches
        .iter()
        .map(|m| {
            let data = match m {
                SemaRitchieParameterArgumentMatch::Regular(_, list_item) => {
                    let sema_expr_idx = list_item.argument_sema_expr_idx();
                    EagerCallInputTraceData::Regular {
                        argument_sema_expr_idx: sema_expr_idx,
                        argument_hir_eager_expr_idx: caller_hir_eager_expr_source_map_data
                            .sema_to_hir_eager_expr_idx(sema_expr_idx),
                    }
                }
                SemaRitchieParameterArgumentMatch::Variadic(_, _) => {
                    todo!()
                }
                SemaRitchieParameterArgumentMatch::Keyed(_, _) => {
                    todo!()
                }
            };
            EagerCallInputTrace::new(
                trace_path,
                trace,
                data,
                caller_sema_expr_region,
                todo!(),
                db,
            )
            .into()
        })
        .collect()
}
