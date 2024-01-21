use crate::{view::settings::TraceDocViewSettings, *};
use husky_code_editor::settings::HasCodeEditorSettings;
use husky_trace_protocol::settings::HasTraceSettings;

/// settings are those users can easily modify and take effects immediately
///
/// configurations are those users are not supposed to modify and might take effects only after a reboot

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TraceDocSettings {
    view: TraceDocViewSettings,
}

pub trait HasTraceDocSettings: HasCodeEditorSettings + HasTraceSettings {
    fn trace_doc_settings(&self) -> &TraceDocSettings;

    fn trace_doc_settings_mut(&mut self) -> &mut TraceDocSettings;
}
