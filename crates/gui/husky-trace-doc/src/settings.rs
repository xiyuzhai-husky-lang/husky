
use husky_code_editor::settings::HasCodeEditorSettings;
use husky_trace_protocol::settings::HasTraceSettings;

pub trait HasTraceViewDocSettings: HasCodeEditorSettings + HasTraceSettings {}
