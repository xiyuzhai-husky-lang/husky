use std::time::Instant;

use crate::*;
use husky_entity_kind::EntityKind;

impl HuskyTracetime {
    pub(crate) fn update(&mut self) {
        self.clear();
        match self.update_root_traces() {
            Ok(_) => (),
            Err(e) => match e.variant() {
                __VMErrorVariant::Normal => todo!(),
                __VMErrorVariant::FromBatch { sample_id } => {
                    self.restriction.set_specific(SampleId(*sample_id));
                    self.update_trace_stalks();
                }
            },
        }
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

    fn update_root_traces(&mut self) -> __VMResult<()> {
        let target_entrance = self.comptime().target_entrance();
        let now = Instant::now();
        let main_feature_repr = self.runtime().main_feature_repr(target_entrance);
        println!(
            "{} milliseconds elapsed for computing main feature",
            now.elapsed().as_millis(),
        );
        let module = self.comptime().module(target_entrance).unwrap();
        let mut root_trace_ids = vec![];
        // add input trace
        root_trace_ids.push(self.new_trace(None, 0, TraceVariant::input(self.runtime())));
        // add module traces
        for (subentity_kind, subentity_route) in
            self.comptime().subentity_kinded_routes(module).iter()
        {
            match subentity_kind {
                EntityKind::Module => {
                    if self.comptime().module_contains_features(*subentity_route) {
                        root_trace_ids.push(self.new_trace(
                            None,
                            0,
                            TraceVariant::Module {
                                route: *subentity_route,
                                file: target_entrance,
                                range: Default::default(),
                            },
                        ))
                    }
                }
                EntityKind::Feature => {
                    let repr = self.runtime().entity_feature_repr(*subentity_route);
                    root_trace_ids.push(self.new_trace(
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
        // add main trace
        root_trace_ids.push(self.new_trace(None, 0, TraceVariant::Main(main_feature_repr)));
        self.root_trace_ids = root_trace_ids;
        self.update_trace_statss()?;
        Ok(())
    }
}
