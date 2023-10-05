use husky_gui::helpers::run_standalone_ui_component;
use husky_trace_protocol::{client_db::TraceDb, *};
use husky_trace_view_doc::{
    doc::{HasTraceViewConfig, TraceViewDoc},
    *,
};
use husky_visual_protocol::mock::MockVisualProtocol;

fn main() {
    let doc: TraceViewDoc<MockVisualProtocol> = TraceViewDoc::new_mock();
    run_standalone_ui_component(doc, MockConfig, ());
}

struct MockConfig;

impl HasTraceViewConfig for MockConfig {}
