use std::sync::Arc;

use crate::*;
#[cfg(feature = "egui")]
use egui::*;
use husky_trace_protocol::{
    cache::TraceCache,
    client::{error::TraceClientResult, TraceClient},
    id::TraceId,
    view::{action::TraceViewActionBuffer, TraceViewData},
};
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::IsVisualComponent;
use ui::IsUiComponent;

pub struct TraceViewDoc<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    trace_client: TraceClient<VisualComponent>,
    action_buffer: TraceViewActionBuffer<VisualComponent>,
}

#[cfg(feature = "egui")]
impl<VisualComponent, Settings, UiActionBuffer> IsUiComponent<egui::Ui, Settings, UiActionBuffer>
    for TraceViewDoc<VisualComponent>
where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    fn update(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut Settings,
        action_buffer: &mut UiActionBuffer,
    ) {
        self.trace_client.update();
        self.render(ui, settings);
        for action in self.action_buffer.take_actions() {
            println!("take action {action:?} from buffer");
            self.trace_client.take_action(action)
        }
    }
}

impl<VisualComponent> TraceViewDoc<VisualComponent>
where
    VisualComponent: IsVisualComponent,
{
    fn render<Settings>(&mut self, ui: &mut Ui, settings: &mut Settings)
    where
        Settings: HasTraceViewDocSettings,
    {
        ui.end_row();
        // ui.label(text)
        let trace_client = &self.trace_client;
        if let Some(e) = trace_client.connection_error() {
            ui.label(RichText::new(e.to_string()).color(Color32::RED));
        }
        if let Some(root_trace_ids) = trace_client.root_trace_ids() {
            let root_trace_ids = root_trace_ids.to_vec();
            render_traces(
                &self.trace_client,
                &root_trace_ids,
                settings,
                &mut self.action_buffer,
                ui,
            )
        } else {
            // todo: render connecting status
        }
    }
}

fn render_traces<VisualComponent, Settings>(
    trace_client: &TraceClient<VisualComponent>,
    trace_ids: &[TraceId],
    settings: &Settings,
    action_buffer: &mut TraceViewActionBuffer<VisualComponent>,
    ui: &mut egui::Ui,
) where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    for &trace_id in trace_ids {
        let entry = &trace_client.cache().unwrap()[trace_id];
        render_trace_view(
            trace_client,
            trace_id,
            entry.view_data(),
            settings,
            action_buffer,
            ui,
        );
        if let Some(subtraces) = entry.subtraces() {
            todo!()
        }
    }
}

fn render_trace_view<VisualComponent, Settings: HasTraceViewDocSettings>(
    trace_client: &TraceClient<VisualComponent>,
    trace_id: TraceId,
    trace_view_data: &TraceViewData,
    settings: &Settings,
    action_buffer: &mut TraceViewActionBuffer<VisualComponent>,
    ui: &mut egui::Ui,
) where
    VisualComponent: IsVisualComponent,
{
    let token_foreground_colors = settings.code_editor_settings().token_foreground_colors();
    ui.horizontal(|ui| {
        if ui.button("+").clicked() {
            action_buffer.push(TraceViewAction::ToggleExpansion { trace_id })
        };
        for token_data in trace_view_data.tokens_data() {
            ui.label(
                RichText::new(token_data.text())
                    .color(token_foreground_colors[token_data.token_class()]),
            );
        }
    });
}

#[cfg(feature = "mock")]
pub type MockTraceViewDoc = TraceViewDoc<()>;

#[cfg(feature = "mock")]
impl TraceViewDoc<()> {
    pub fn new_mock(tokio_runtime: Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            trace_client: TraceClient::new_mock(tokio_runtime),
            action_buffer: Default::default(),
        }
    }
}
