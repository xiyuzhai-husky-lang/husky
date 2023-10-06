use husky_code_editor::settings::HasCodeEditorSettings;
use husky_gui::helpers::run_standalone_ui_component;
use husky_trace_protocol::{cache::TraceCache, *};
use husky_trace_view_doc::{doc::TraceViewDoc, settings::HasTraceViewSettings, *};
use husky_visual_protocol::mock::MockVisualProtocol;

#[tokio::main]
async fn main() {
    let doc: TraceViewDoc<MockVisualProtocol> = TraceViewDoc::new_mock();
    run_standalone_ui_component(doc, MockConfig, ());
}

struct MockConfig;

impl HasCodeEditorSettings for MockConfig {
    fn code_editor_settings(&self) -> &husky_code_editor::settings::CodeEditorSettings {
        todo!()
    }
}

impl HasTraceViewSettings for MockConfig {}
