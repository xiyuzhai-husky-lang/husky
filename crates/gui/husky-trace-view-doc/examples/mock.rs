use std::sync::Arc;

use husky_code_editor::settings::HasCodeEditorSettings;
use husky_gui::helpers::run_standalone_ui_component;
use husky_trace_protocol::{
    cache::TraceCache,
    settings::{HasTraceSettings, TraceSettings},
    *,
};
use husky_trace_view_doc::{doc::TraceViewDoc, settings::HasTraceViewDocSettings, *};
use husky_visual_protocol::mock::MockVisualProtocol;

fn main() {
    let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let tokio_runtime = Arc::new(tokio_runtime);
    let doc: TraceViewDoc<MockVisualProtocol, ()> = TraceViewDoc::new_mock(tokio_runtime);
    run_standalone_ui_component(doc, MockConfig, ());
}

struct MockConfig;

impl HasCodeEditorSettings for MockConfig {
    fn code_editor_settings(&self) -> &husky_code_editor::settings::CodeEditorSettings {
        todo!()
    }
}

impl HasTraceSettings for MockConfig {
    fn trace_settings(&self) -> &TraceSettings {
        todo!()
    }
}

impl HasTraceViewDocSettings for MockConfig {}
