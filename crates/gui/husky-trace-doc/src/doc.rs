use std::sync::Arc;

use crate::{view::TraceDocView, *};
#[cfg(feature = "egui")]
use egui::*;
use husky_gui::helpers::repaint_signal::EguiRepaintSignal;

use husky_trace_protocol::{
    client::{TraceClient},
    view::{action::TraceViewActionBuffer},
};

use husky_visual_protocol::IsVisualComponent;
use notify::Notify;
use ui::IsUiComponent;

pub struct TraceDoc<VisualComponent, RepaintSignal>
where
    VisualComponent: IsVisualComponent,
    RepaintSignal: Notify,
{
    trace_client: TraceClient<VisualComponent, RepaintSignal>,
    action_buffer: TraceViewActionBuffer<VisualComponent>,
}

#[cfg(feature = "egui")]
impl<VisualComponent, Settings, UiActionBuffer> IsUiComponent<egui::Ui, Settings, UiActionBuffer>
    for TraceDoc<VisualComponent, EguiRepaintSignal>
where
    VisualComponent: IsVisualComponent,
    Settings: HasTraceViewDocSettings,
{
    fn update(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut Settings,
        _action_buffer: &mut UiActionBuffer,
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
impl<VisualComponent> TraceDoc<VisualComponent, EguiRepaintSignal>
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
        if let Some(trace_cache) = trace_client.opt_cache() {
            TraceDocView::new(trace_cache, &mut self.action_buffer, ui, settings).ui(ui);
        } else {
            // todo: render connecting status
        }
    }
}

#[cfg(feature = "mock")]
pub type MockTraceDoc = TraceDoc<(), EguiRepaintSignal>;

#[cfg(feature = "mock")]
impl TraceDoc<(), EguiRepaintSignal> {
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
