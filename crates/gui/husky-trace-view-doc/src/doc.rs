use crate::*;
#[cfg(feature = "egui")]
use egui::*;
use husky_trace_protocol::{
    cache::TraceCache,
    client::TraceClientCache,
    id::TraceIdRange,
    view::{action::TraceViewActionBuffer, TraceViewData},
};
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::IsVisualProtocol;
use ui::IsUiComponent;

pub struct TraceViewDoc<VisualProtocol: IsVisualProtocol> {
    text: String,
    trace_client_cache: TraceClientCache<VisualProtocol>,
    buffer_action: TraceViewActionBuffer,
}

#[cfg(feature = "egui")]
impl<VisualProtocol: IsVisualProtocol, Settings: HasTraceViewSettings, UiActionBuffer>
    IsUiComponent<egui::Ui, Settings, UiActionBuffer> for TraceViewDoc<VisualProtocol>
{
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut Settings,
        action_buffer: &mut UiActionBuffer,
    ) {
        ui.text_edit_singleline(&mut self.text);
        ui.end_row();
        // ui.label(text)
        self.render_traces(self.trace_client_cache.root_trace_ids(), ui, settings)
    }
}

impl<VisualProtocol: IsVisualProtocol> TraceViewDoc<VisualProtocol> {
    fn render_traces<Settings: HasTraceViewSettings>(
        &mut self,
        trace_id_range: TraceIdRange,
        ui: &mut egui::Ui,
        settings: &Settings,
    ) {
        for trace_entry in &self.trace_client_cache[trace_id_range] {
            self.render_trace_view(trace_entry.view_data(), ui, settings);
            if let Some(subtraces) = trace_entry.subtraces() {
                todo!()
            }
        }
    }

    fn render_trace_view<Settings: HasTraceViewSettings>(
        &self,
        trace_view_data: &TraceViewData,
        ui: &mut egui::Ui,
        settings: &Settings,
    ) {
        let token_foreground_colors = settings.code_editor_settings().token_foreground_colors();
        ui.horizontal(|ui| {
            for token_data in trace_view_data.tokens_data() {
                ui.label(
                    RichText::new(token_data.text())
                        .color(token_foreground_colors[token_data.token_class()]),
                );
            }
        });
    }
}

#[cfg(feature = "mock")]
pub type MockTraceViewDoc = TraceViewDoc<MockVisualProtocol>;

#[cfg(feature = "mock")]
impl TraceViewDoc<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            text: "hello".to_string(),
            trace_client_cache: TraceClientCache::new_mock(),
            buffer_action: Default::default(),
        }
    }
}
