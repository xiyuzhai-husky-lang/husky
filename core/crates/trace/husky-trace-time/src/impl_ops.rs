use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub(crate) fn update(&mut self) {
        self.clear();
        self.update_root_traces();
    }

    fn clear(&mut self) {
        // replace this with diff, try to make the trace tree look the same across code change
        self.trace_nodes.clear();
        self.opt_active_trace_id = None;
        self.root_trace_ids.clear();
        self.subtrace_ids_map.clear();
        self.figure_controls.clear();
    }

    fn update_root_traces(&mut self) {
        let main_file = self.runtime_singleton.compile_time().main_file();
        let main_feature_repr = self.runtime_singleton.main_feature_repr(main_file);
        self.root_trace_ids = vec![self.new_trace(None, 0, TraceVariant::Main(main_feature_repr))];
    }
}
