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
        self.root_traces.clear();
        self.subtraces_map.clear();
        self.figures.clear();
        self.figure_controls.clear();
    }

    fn update_root_traces(&mut self) {
        let main_file = self.runtime.compile_time().main_file();
        self.root_traces = vec![self
            .new_trace(
                None,
                0,
                TraceVariant::Main(
                    self.runtime
                        .compile_time()
                        .main_feature_repr(main_file)
                        .unwrap(),
                ),
                &self.runtime.compile_time().text(main_file).unwrap(),
            )
            .id()];
    }
}
