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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LazyExprTracePathData {
    biological_parent_path: TracePath,
    essence: LazyExprEssence,
    disambiguator: TracePathDisambiguator<LazyExprEssence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LazyExprEssence {
    Haha,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExprTraceData {
    path: TracePath,
    biological_parent: TraceId,
    sema_expr_idx: SemaExprIdx,
    hir_lazy_expr_idx: Option<HirLazyExprIdx>,
    sema_expr_region: SemaExprRegion,
    hir_lazy_expr_region: HirLazyExprRegion,
    hir_lazy_expr_source_map: HirLazyExprSourceMap,
}

impl TraceId {
    pub(crate) fn new_lazy_expr(
        biological_parent_path: TracePath,
        biological_parent: TraceId,
        sema_expr_idx: SemaExprIdx,
        hir_lazy_expr_idx: Option<HirLazyExprIdx>,
        sema_expr_region: SemaExprRegion,
        hir_lazy_expr_region: HirLazyExprRegion,
        hir_lazy_expr_source_map: HirLazyExprSourceMap,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<LazyExprEssence>,
        db: &::salsa::Db,
    ) -> Self {
        let essence = LazyExprEssence::Haha;
        let path = TracePath::new(
            LazyExprTracePathData {
                biological_parent_path,
                essence: essence.clone(),
                disambiguator: lazy_expr_trace_path_registry.issue(essence),
            },
            db,
        );
        TraceId::new(
            path,
            LazyExprTraceData {
                path,
                biological_parent: biological_parent.into(),
                sema_expr_idx,
                hir_lazy_expr_idx,
                sema_expr_region,
                hir_lazy_expr_region,
                hir_lazy_expr_source_map,
            }
            .into(),
            db,
        )
    }
}

impl LazyExprTraceData {
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
        let Some(hir_eager_expr_idx) = self.hir_lazy_expr_idx else {
            return false;
        };
        match self.hir_lazy_expr_region.hir_lazy_expr_arena(db)[hir_eager_expr_idx] {
            HirLazyExprData::FunctionFnItemCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::AssociatedFunctionFnCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::MethodFnCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::AssociatedFn { path } => path.hir_defn(db).is_some(),
            HirLazyExprData::FunctionGnItemCall { path, .. } => path.hir_defn(db).is_some(),
            HirLazyExprData::Block { stmts } => unreachable!(),
            _ => false,
        }
    }

    pub(super) fn subtraces(&self, trace: TraceId, db: &::salsa::Db) -> Vec<TraceId> {
        let biological_parent_path = self.path;
        let biological_parent = trace;
        let sema_expr_idx = self.sema_expr_idx;
        let Some(hir_eager_expr_idx) = self.hir_lazy_expr_idx else {
            return vec![];
        };
        let sema_expr_region_data = self.sema_expr_region.data(db);
        let hir_lazy_expr_source_map_data = self.hir_lazy_expr_source_map.data(db);
        match self.hir_lazy_expr_region.hir_lazy_expr_arena(db)[hir_eager_expr_idx] {
            HirLazyExprData::FunctionFnItemCall { path, .. } => {
                let SemaExprData::FunctionFnCall {
                    ref ritchie_parameter_argument_matches,
                    ..
                } = sema_expr_idx.data(sema_expr_region_data.sema_expr_arena())
                else {
                    unreachable!()
                };
                let mut subtraces: Vec<TraceId> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    TraceId::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
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
                let mut subtraces: Vec<TraceId> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    TraceId::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
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
                let mut subtraces: Vec<TraceId> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    TraceId::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
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
                let mut subtraces: Vec<TraceId> = fn_call_lazy_expr_trace_input_traces(
                    biological_parent_path,
                    biological_parent,
                    ritchie_parameter_argument_matches,
                    hir_lazy_expr_source_map_data,
                    db,
                );
                subtraces.push(
                    TraceId::new_lazy_call(
                        biological_parent_path,
                        biological_parent,
                        path.into(),
                        db,
                    )
                    .into(),
                );
                subtraces
            }
            _ => vec![],
        }
    }

    pub(super) fn val_repr(&self, trace_id: TraceId, db: &::salsa::Db) -> Option<ValRepr> {
        let val_repr_expansion = trace_val_repr_expansion(db, trace_id);
        val_repr_expansion
            .hir_lazy_expr_val_repr_map(db)
            .get(self.hir_lazy_expr_idx?)
            .copied()
    }

    pub(super) fn val_repr_expansion(&self, db: &::salsa::Db) -> ValReprExpansion {
        self.biological_parent.val_repr_expansion(db)
    }
}

fn fn_call_lazy_expr_trace_input_traces(
    trace_path: TracePath,
    trace: TraceId,
    ritchie_parameter_argument_matches: &[SemaRitchieParameterArgumentMatch],
    hir_lazy_expr_source_map_data: &HirLazyExprSourceMapData,
    db: &::salsa::Db,
) -> Vec<TraceId> {
    ritchie_parameter_argument_matches
        .iter()
        .map(|m| {
            let data = match m {
                SemaRitchieParameterArgumentMatch::Regular(_, list_item) => {
                    let sema_expr_idx = list_item.argument_sema_expr_idx();
                    LazyCallInputSketch::Regular {
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
            TraceId::new_lazy_call_input(trace_path, trace, data, db).into()
        })
        .collect()
}
