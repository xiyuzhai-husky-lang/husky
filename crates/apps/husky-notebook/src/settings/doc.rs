use husky_code_editor::settings::{CodeEditorSettings, HasCodeEditorSettings};
use husky_trace_doc::settings::HasTraceDocSettings;
use husky_trace_protocol::settings::{HasTraceSettings, TraceSettings};

#[derive(Default, PartialEq, Eq)]
pub struct HuskyNotebookDocSettings {
    code_editor_settings: CodeEditorSettings,
    trace_settings: TraceSettings,
}

impl HasTraceSettings for HuskyNotebookDocSettings {
    fn trace_settings(&self) -> &TraceSettings {
        &self.trace_settings
    }
}

impl HasTraceDocSettings for HuskyNotebookDocSettings {
    fn trace_doc_settings(&self) -> &husky_trace_doc::settings::TraceDocSettings {
        todo!()
    }

    fn trace_doc_settings_mut(&mut self) -> &mut husky_trace_doc::settings::TraceDocSettings {
        todo!()
    }
}

impl HasCodeEditorSettings for HuskyNotebookDocSettings {
    fn code_editor_settings(&self) -> &husky_code_editor::settings::CodeEditorSettings {
        &self.code_editor_settings
    }
}
