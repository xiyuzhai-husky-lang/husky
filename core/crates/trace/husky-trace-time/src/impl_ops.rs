use std::time::Instant;

use crate::*;
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
        let main_file = self.comptime().main_file();
        let now = Instant::now();
        let main_feature_repr = self.runtime().main_feature_repr(main_file);
        println!(
            "{} milliseconds elapsed for computing main feature",
            now.elapsed().as_millis(),
        );
        let module = self.comptime().module(main_file).unwrap();
        let mut root_trace_ids = vec![];
        for submodule in self.comptime().submodules(module).iter() {
            root_trace_ids.push(self.new_trace(
                None,
                0,
                TraceVariant::Module {
                    route: *submodule,
                    file: main_file,
                    range: Default::default(),
                },
            ))
        }
        root_trace_ids.push(self.new_trace(None, 0, TraceVariant::Main(main_feature_repr)));
        self.root_trace_ids = root_trace_ids
    }
}
