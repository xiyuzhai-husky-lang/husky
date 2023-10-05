use husky_gui::helpers::run_standalone_ui_component;
use husky_trace_protocol::*;
use husky_trace_view_doc::{
    doc::{HasTraceViewConfig, TraceViewDoc},
    *,
};
use husky_visual_protocol::mock::MockVisualProtocol;

fn main() {
    let doc: TraceViewDoc<MockVisualProtocol> = todo!();
    run_standalone_ui_component(doc, MockConfig);
}

struct MockConfig;

impl HasTraceViewConfig for MockConfig {}
