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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(debug_assertions, salsa::debug_with_db(db = TraceDb))]
pub struct EagerExprTracePathData {
    biological_parent_path: TracePath,
    essence: EagerExprEssence,
    disambiguator: TracePathDisambiguator<EagerExprEssence>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(debug_assertions, salsa::debug_with_db(db = TraceDb))]
pub enum EagerExprEssence {
    Haha,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(debug_assertions, salsa::debug_with_db(db = TraceDb))]
pub struct EagerExprTraceData {
    path: TracePath,
    biological_parent: Trace,
    sema_expr_idx: SemaExprIdx,
    hir_eager_expr_idx: Option<HirEagerExprIdx>,
    sema_expr_region: SemaExprRegion,
    hir_eager_expr_region: HirEagerExprRegion,
    hir_eager_expr_source_map: HirEagerExprSourceMap,
}

impl Trace {
    pub(crate) fn new_eager_expr(
        biological_parent_path: TracePath,
        biological_parent: Trace,
        sema_expr_idx: SemaExprIdx,
        hir_eager_expr_idx: Option<HirEagerExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_eager_expr_region: HirEagerExprRegion,
        hir_eager_expr_source_map: HirEagerExprSourceMap,
        eager_expr_trace_path_registry: &mut TracePathRegistry<EagerExprEssence>,
        db: &::salsa::Db,
    ) -> Self {
        let essence = EagerExprEssence::Haha;
        let path = TracePath::new(
            EagerExprTracePathData {
                biological_parent_path,
                essence: essence.clone(),
                disambiguator: eager_expr_trace_path_registry.issue(essence),
            },
            db,
        );
        Trace::new(
            path,
            EagerExprTraceData {
                path,
                biological_parent: biological_parent.into(),
                sema_expr_idx,
                hir_eager_expr_idx,
                sema_expr_region,
                hir_eager_expr_region,
                hir_eager_expr_source_map,
            }
            .into(),
            db,
        )
    }
}

impl EagerExprTraceData {
    pub(super) fn view_lines(&self, db: &::salsa::Db) -> TraceViewLines {
        let sema_expr_region = self.sema_expr_region;
        let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
        let sema_expr_range_region_data = sema_expr_range_region.data(db);
        let region_path = sema_expr_region.path(db);
        let regional_token_idx_range = sema_expr_range_region_data[self.sema_expr_idx];
        let token_idx_range = regional_token_idx_range
            .token_idx_range(region_path.regional_token_idx_base(db).unwrap());
        TraceViewLines::new(
            region_path.module_path(db),
            token_idx_range,
            VoidAssociatedTraceRegistry,
            db,
        )
    }

    pub(super) fn have_subtraces(&self, db: &::salsa::Db) -> bool {
        use husky_hir_defn::defn::HasHirDefn;
        let Some(hir_eager_expr_idx) = self.hir_eager_expr_idx else {
            return false;
        };
        match self.hir_eager_expr_region.hir_eager_expr_arena(db)[hir_eager_expr_idx] {
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

    fn eager_expr_trace_subtraces(&self, trace: Trace, db: &::salsa::Db) -> Vec<Trace> {
        use husky_syn_defn::HasSynDefn;
        let biological_parent_path = self.path;
        let biological_parent = trace;
        let sema_expr_idx = self.sema_expr_idx;
        let Some(hir_eager_expr_idx) = self.hir_eager_expr_idx else {
            return vec![];
        };
        let caller_sema_expr_region = self.sema_expr_region;
        let caller_sema_expr_region_data = caller_sema_expr_region.data(db);
        let hir_eager_expr_source_map_data = self.hir_eager_expr_source_map.data(db);
        match self.hir_eager_expr_region.hir_eager_expr_arena(db)[hir_eager_expr_idx] {
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
                let syn_defn = path.syn_defn(db).unwrap();
                let syn_decl = syn_defn.decl(db);
                let callee_syn_expr_region = syn_decl.syn_expr_region(db);
                let mut subtraces = fn_call_eager_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    caller_sema_expr_region,
                    hir_eager_expr_source_map_data,
                    callee_syn_expr_region,
                    db,
                );
                subtraces.push(
                    Trace::new_eager_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
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
                let syn_defn = path.syn_defn(db).expect("no syn error at trace stage");
                let syn_decl = syn_defn.decl(db);
                // todo: syn_decl.parenate_parameters(db);
                let callee_syn_expr_region = syn_decl.syn_expr_region(db);
                let mut subtraces = fn_call_eager_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    caller_sema_expr_region,
                    hir_eager_expr_source_map_data,
                    callee_syn_expr_region,
                    db,
                );
                subtraces.push(
                    Trace::new_eager_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
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
                let syn_defn = path.syn_defn(db).expect("no syn error at trace stage");
                let syn_decl = syn_defn.decl(db);
                let callee_syn_expr_region = syn_decl.syn_expr_region(db);
                let mut subtraces = fn_call_eager_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    caller_sema_expr_region,
                    hir_eager_expr_source_map_data,
                    callee_syn_expr_region,
                    db,
                );
                subtraces.push(
                    Trace::new_eager_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
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
}

fn fn_call_eager_expr_trace_input_traces(
    trace_path: TracePath,
    trace: Trace,
    ritchie_parameter_argument_matches: &[SemaRitchieParameterArgumentMatch],
    caller_sema_expr_region: SemaExprRegion,
    caller_hir_eager_expr_source_map_data: &HirEagerExprSourceMapData,
    callee_syn_expr_region: SynExprRegion,
    db: &::salsa::Db,
) -> Vec<Trace> {
    ritchie_parameter_argument_matches
        .iter()
        .map(|m| {
            let data = match m {
                SemaRitchieParameterArgumentMatch::Regular(_, list_item) => {
                    let sema_expr_idx = list_item.argument_sema_expr_idx();
                    EagerCallInputSketch::Regular {
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
            Trace::new_eager_call_input(
                trace_path,
                trace,
                data,
                caller_sema_expr_region,
                callee_syn_expr_region,
                db,
            )
            .into()
        })
        .collect()
}
