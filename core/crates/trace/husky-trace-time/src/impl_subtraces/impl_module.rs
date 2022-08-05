use crate::*;
use entity_kind::EntityKind;
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
                EntityKind::Module => subtrace_ids.push(self.new_trace(
                    None,
                    0,
                    TraceVariant::Module {
                        route: *subentity_route,
                        file: module_file,
                        range: Default::default(),
                    },
                )),
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
// let main_file = self.comptime().main_file();
// let main_feature_repr = self.runtime().main_feature_repr(main_file);
// println!(
//     "{} milliseconds elapsed for computing main feature",
//     now.elapsed().as_millis(),
// );
// let module = self.comptime().module(main_file).unwrap();

// self.root_trace_ids = root_trace_ids
