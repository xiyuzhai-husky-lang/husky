use std::time::Instant;

use crate::*;
use entity_kind::EntityKind;
use husky_text::HuskyText;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub(crate) fn update(&mut self) {
        self.clear();
        self.update_root_traces();
    }

    fn clear(&mut self) {
        // replace this with diff, try to make the trace tree look the same across code change
        self.restriction = Default::default();
        self.pins.clear();
        self.trace_nodes.clear();
        self.opt_active_trace_id = None;
        self.trace_stalks.clear();
        self.root_trace_ids.clear();
        self.subtrace_ids_map.clear();
        self.figure_controls.clear();
    }

    fn update_root_traces(&mut self) {
        let target_entrance = self.comptime().target_entrance();
        let now = Instant::now();
        let main_feature_repr = self.runtime().main_feature_repr(target_entrance);
        println!(
            "{} milliseconds elapsed for computing main feature",
            now.elapsed().as_millis(),
        );
        let module = self.comptime().module(target_entrance).unwrap();
        let mut root_trace_ids = vec![];
        for (subentity_kind, subentity_route) in
            self.comptime().subentity_kinded_routes(module).iter()
        {
            match subentity_kind {
                EntityKind::Module => root_trace_ids.push(self.new_trace(
                    None,
                    0,
                    TraceVariant::Module {
                        route: *subentity_route,
                        file: target_entrance,
                        range: Default::default(),
                    },
                )),
                EntityKind::Feature => todo!(),
                _ => (),
            }
        }
        root_trace_ids.push(self.new_trace(None, 0, TraceVariant::Main(main_feature_repr)));
        self.root_trace_ids = root_trace_ids
    }
}
