use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_route::EntityRoutePtr;
use husky_feature_eval::FeatureEvaluator;
use std::sync::Arc;

impl HuskyTraceTime {
    pub(super) fn module_subtraces(
        &mut self,
        trace: &Trace,
        module: EntityRoutePtr,
    ) -> Vec<TraceId> {
        let mut subtrace_ids = vec![];
        let module_file = self.comptime().module_file(module).unwrap();
        for (subentity_kind, subentity_route) in
            self.comptime().subentity_kinded_routes(module).iter()
        {
            match subentity_kind {
                EntityKind::Module => {
                    if self.comptime().module_contains_features(*subentity_route) {
                        subtrace_ids.push(self.new_trace(
                            None,
                            0,
                            TraceVariant::Module {
                                route: *subentity_route,
                                file: module_file,
                                range: Default::default(),
                            },
                        ))
                    }
                }
                EntityKind::Feature => {
                    let repr = self.runtime().entity_feature_repr(*subentity_route);
                    subtrace_ids.push(self.new_trace(
                        None,
                        0,
                        TraceVariant::EntityFeature {
                            route: *subentity_route,
                            repr,
                        },
                    ))
                }
                _ => (),
            }
        }
        subtrace_ids
    }
}
// let target_entrance = self.comptime().target_entrance();
// let main_feature_repr = self.runtime().main_feature_repr(target_entrance);
// println!(
//     "{} milliseconds elapsed for computing main feature",
//     now.elapsed().as_millis(),
// );
// let module = self.comptime().module(target_entrance).unwrap();

// self.root_trace_ids = root_trace_ids
