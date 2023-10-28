use std::sync::Arc;

use crate::*;
#[cfg(feature = "egui")]
use egui::*;
use husky_gui::helpers::repaint_signal::EguiRepaintSignal;
use husky_print_utils::p;
use husky_trace_protocol::{
    cache::{TraceCache, TraceCacheEntry},
    client::{error::TraceClientResult, TraceClient},
    id::TraceId,
    view::{action::TraceViewActionBuffer, TraceViewData},
};
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::IsVisualComponent;
use notify::Notify;
use ui::IsUiComponent;

pub struct TraceViewDoc<VisualComponent, RepaintSignal>
where
    VisualComponent: IsVisualComponent,
    RepaintSignal: Notify,
{
    trace_client: TraceClient<VisualComponent, RepaintSignal>,
    action_buffer: TraceViewActionBuffer<VisualComponent>,
}

#[cfg(feature = "egui")]
impl<VisualComponent, Settings, UiActionBuffer> IsUiComponent<egui::Ui, Settings, UiActionBuffer>
    for TraceViewDoc<VisualComponent, EguiRepaintSignal>
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
            match self.trace_client.take_view_action(action) {
                Ok(_) => (),
                Err(e) => println!("e = {e} while take action"),
            }
        }
    }
}

#[cfg(feature = "egui")]
impl<VisualComponent> TraceViewDoc<VisualComponent, EguiRepaintSignal>
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

#[cfg(feature = "egui")]
fn render_traces<VisualComponent, Settings>(
    trace_client: &TraceClient<VisualComponent, EguiRepaintSignal>,
    trace_ids: &[TraceId],
    settings: &Settings,
    action_buffer: &mut TraceViewActionBuffer<VisualComponent>,
    ui: &mut egui::Ui,
) where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    let glyph_width = ui.fonts(|f| f.glyph_width(&TextStyle::Monospace.resolve(ui.style()), ' '));
    render_traces_aux(
        trace_client,
        trace_ids,
        glyph_width,
        settings,
        action_buffer,
        ui,
    )
}

#[cfg(feature = "egui")]
fn render_traces_aux<VisualComponent, Settings>(
    trace_client: &TraceClient<VisualComponent, EguiRepaintSignal>,
    trace_ids: &[TraceId],
    glyph_width: f32,
    settings: &Settings,
    action_buffer: &mut TraceViewActionBuffer<VisualComponent>,
    ui: &mut egui::Ui,
) where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    let glyph_width = ui.fonts(|f| f.glyph_width(&TextStyle::Monospace.resolve(ui.style()), ' '));
    for &trace_id in trace_ids {
        let entry = &trace_client.opt_cache().unwrap()[trace_id];
        render_trace_view(
            trace_client,
            trace_id,
            entry,
            glyph_width,
            settings,
            action_buffer,
            ui,
        );
        if entry.expanded()
            && let Some(subtrace_ids) = entry.subtrace_ids()
        {
            render_traces(trace_client, subtrace_ids, settings, action_buffer, ui)
        }
    }
}

#[cfg(feature = "egui")]
fn render_trace_view<VisualComponent, Settings: HasTraceViewDocSettings>(
    trace_client: &TraceClient<VisualComponent, EguiRepaintSignal>,
    trace_id: TraceId,
    entry: &TraceCacheEntry,
    glyph_width: f32,
    settings: &Settings,
    action_buffer: &mut TraceViewActionBuffer<VisualComponent>,
    ui: &mut egui::Ui,
) where
    VisualComponent: IsVisualComponent,
{
    use husky_trace_protocol::view::SeparationAfter;

    let token_foreground_colors = settings.code_editor_settings().token_foreground_colors();
    ui.horizontal(|ui| {
        let button_text = match entry.expanded() {
            true => "-",
            false => "+",
        };
        let button_text = RichText::new(button_text).family(FontFamily::Monospace);
        if ui.button(button_text).clicked() {
            action_buffer.push(TraceViewAction::ToggleExpansion { trace_id })
        };
        let mut new_line = false;
        for token_data in entry.view_data().tokens_data() {
            if new_line {
                todo!()
            }
            ui.spacing_mut().item_spacing.x = match token_data.separation_after() {
                SeparationAfter::SameLine { spaces } => (spaces as f32) * glyph_width,
                SeparationAfter::NextLine { indent } => {
                    new_line = true;
                    0.
                }
                SeparationAfter::Eof => 0.,
            };
            ui.label(
                RichText::new(token_data.text())
                    .family(FontFamily::Monospace)
                    .color(token_foreground_colors[token_data.token_class()]),
            );
        }
    });
}

#[cfg(feature = "mock")]
pub type MockTraceViewDoc = TraceViewDoc<(), EguiRepaintSignal>;

#[cfg(feature = "mock")]
impl TraceViewDoc<(), EguiRepaintSignal> {
    pub fn new_mock(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        repaint_signal: EguiRepaintSignal,
    ) -> Self {
        Self {
            trace_client: TraceClient::new_mock(tokio_runtime, repaint_signal),
            action_buffer: Default::default(),
        }
    }
}
