use crate::*;
use husky_entity_taxonomy::EntityKind;
use husky_ethereal_term::EtherealTerm;

impl Debugtime {
    pub(super) fn module_subtraces(&mut self, trace: &Trace, module: EtherealTerm) -> Vec<TraceId> {
        todo!()
        // let mut subtrace_ids = vec![];
        // let module_file = self.runtime().module_file(module).unwrap();
        // for (subentity_kind, subentity_route) in
        //     self.runtime().subentity_kinded_routes(module).iter()
        // {
        //     match subentity_kind {
        //         EntityKind::Module => {
        //             if self.runtime().module_contains_features(*subentity_route) {
        //                 subtrace_ids.push(self.new_trace(
        //                     Some(trace.id()),
        //                     0,
        //                     TraceVariant::Module {
        //                         route: *subentity_route,
        //                         file: module_file,
        //                         range: Default::default(),
        //                     },
        //                 ))
        //             }
        //         }
        //         EntityKind::Feature => {
        //             let repr = self.runtime().entity_feature_repr(*subentity_route);
        //             subtrace_ids.push(self.new_trace(
        //                 Some(trace.id()),
        //                 0,
        //                 TraceVariant::EntityFeature {
        //                     route: *subentity_route,
        //                     repr,
        //                 },
        //             ))
        //         }
        //         _ => (),
        //     }
        // }
        // subtrace_ids
    }
}
