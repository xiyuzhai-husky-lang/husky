use crate::*;
#[cfg(feature = "egui")]
use egui::*;
use husky_trace_protocol::{client_db::TraceDb, view::action::TraceViewActionBuffer};
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::IsVisualProtocol;
use ui::IsUiComponent;

pub struct TraceViewDoc<VisualProtocol: IsVisualProtocol> {
    text: String,
    trace_db: TraceDb<VisualProtocol>,
    buffer_action: TraceViewActionBuffer,
}

#[cfg(feature = "egui")]
impl<VisualProtocol: IsVisualProtocol, UiComponentConfig: HasTraceViewConfig, UiActionBuffer>
    IsUiComponent<egui::Ui, UiComponentConfig, UiActionBuffer> for TraceViewDoc<VisualProtocol>
{
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        config: &UiComponentConfig,
        action_buffer: &mut UiActionBuffer,
    ) {
        ui.text_edit_singleline(&mut self.text);
        ui.end_row();
        // ui.label(text)
        for _ in 0..100 {
            ui.horizontal(|ui| {
                ui.label(RichText::new(self.text.clone()).color(Color32::RED));
                ui.label(RichText::new(self.text.clone()).color(Color32::GREEN));
                ui.label(RichText::new(self.text.clone()).color(Color32::BLUE));
                ui.label(RichText::new(self.text.clone()).color(Color32::LIGHT_BLUE));
                ui.label(RichText::new(self.text.clone()).color(Color32::TEMPORARY_COLOR));
            });
        }
    }
}

pub trait HasTraceViewConfig {}

#[cfg(feature = "mock")]
pub type MockTraceViewDoc = TraceViewDoc<MockVisualProtocol>;

#[cfg(feature = "mock")]
impl TraceViewDoc<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            text: "hello".to_string(),
            trace_db: TraceDb::new_mock(),
            buffer_action: Default::default(),
        }
    }
}
