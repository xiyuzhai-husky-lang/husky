use crate::*;
#[cfg(feature = "egui")]
use egui::*;
use husky_trace_protocol::{
    cache::TraceCache,
    client::{error::TraceClientResult, TraceClient},
    id::TraceIdRange,
    view::{action::TraceViewActionBuffer, TraceViewData},
};
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::IsVisualProtocol;
use ui::IsUiComponent;

pub struct TraceViewDoc<VisualProtocol: IsVisualProtocol> {
    trace_client: TraceClient<VisualProtocol>,
    buffer_action: TraceViewActionBuffer,
}

#[cfg(feature = "egui")]
impl<VisualProtocol: IsVisualProtocol, Settings: HasTraceViewDocSettings, UiActionBuffer>
    IsUiComponent<egui::Ui, Settings, UiActionBuffer> for TraceViewDoc<VisualProtocol>
{
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut Settings,
        action_buffer: &mut UiActionBuffer,
    ) {
        ui.end_row();
        // ui.label(text)
        let trace_client = &self.trace_client;
        if let Some(e) = trace_client.connection_error() {
            ui.label(RichText::new(e.to_string()).color(Color32::RED));
        }
        if let Some(root_trace_ids) = trace_client.root_trace_ids() {
            render_traces(trace_client, root_trace_ids, ui, settings)
        } else {
            // todo: render connecting status
        }
    }
}

fn render_traces<VisualProtocol: IsVisualProtocol, Settings: HasTraceViewDocSettings>(
    trace_client: &TraceClient<VisualProtocol>,
    trace_id_range: TraceIdRange,
    ui: &mut egui::Ui,
    settings: &Settings,
) {
    for trace_entry in &trace_client[trace_id_range] {
        render_trace_view(trace_entry.view_data(), ui, settings);
        if let Some(subtraces) = trace_entry.subtraces() {
            todo!()
        }
    }
}

fn render_trace_view<Settings: HasTraceViewDocSettings>(
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

#[cfg(feature = "mock")]
pub type MockTraceViewDoc = TraceViewDoc<MockVisualProtocol>;

#[cfg(feature = "mock")]
impl TraceViewDoc<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            trace_client: TraceClient::new_mock(),
            buffer_action: Default::default(),
        }
    }
}
