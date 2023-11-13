

use husky_code_editor::settings::HasCodeEditorSettings;

use husky_trace_doc::{settings::HasTraceViewDocSettings};
use husky_trace_protocol::{
    settings::{HasTraceSettings, TraceSettings},
};


fn main() {
    todo!()
    // let tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    // let tokio_runtime = Arc::new(tokio_runtime);
    // let doc: TraceViewDoc<(), EguiRepaintSignal> = TraceViewDoc::new_mock(tokio_runtime);
    // run_standalone_ui_component(doc, MockConfig, ());
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
