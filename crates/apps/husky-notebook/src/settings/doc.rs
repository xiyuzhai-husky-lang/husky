use husky_code_editor::settings::{CodeEditorSettings, HasCodeEditorSettings};
use husky_trace_view_doc::settings::HasTraceViewSettings;

#[derive(Default, PartialEq, Eq)]
pub struct HuskyNotebookDocSettings {
    code_editor_settings: CodeEditorSettings,
}

impl HasTraceViewSettings for HuskyNotebookDocSettings {}

impl HasCodeEditorSettings for HuskyNotebookDocSettings {
    fn code_editor_settings(&self) -> &husky_code_editor::settings::CodeEditorSettings {
        todo!()
    }
}
