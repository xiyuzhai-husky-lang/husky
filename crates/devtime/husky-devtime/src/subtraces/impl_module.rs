use crate::*;
use husky_entity_taxonomy::EntityKind;
use husky_ethereal_term::EtherealTerm;

impl<Task: IsTask> Devtime<Task> {
    pub(super) fn module_subtraces(&mut self, trace: &Trace, module: EtherealTerm) -> Vec<TraceId> {
        todo!()
        // let mut subtrace_ids = vec![];
        // let module_file = self.runtime().module_file(module).unwrap();
        // for (subitem_kind, subitem_route) in
        //     self.runtime().subitem_kinded_routes(module).iter()
        // {
        //     match subitem_kind {
        //         EntityKind::Module => {
        //             if self.runtime().module_contains_features(*subitem_route) {
        //                 subtrace_ids.push(self.new_trace(
        //                     Some(trace.id()),
        //                     0,
        //                     TraceVariant::Module {
        //                         route: *subitem_route,
        //                         file: module_file,
        //                         range: Default::default(),
        //                     },
        //                 ))
        //             }
        //         }
        //         EntityKind::Feature => {
        //             let repr = self.runtime().item_feature_repr(*subitem_route);
        //             subtrace_ids.push(self.new_trace(
        //                 Some(trace.id()),
        //                 0,
        //                 TraceVariant::EntityFeature {
        //                     route: *subitem_route,
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
