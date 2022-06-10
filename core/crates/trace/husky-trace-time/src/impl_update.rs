use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl HuskyTraceTime {
    pub(crate) fn update(&mut self) {
        self.update_root_traces();
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
