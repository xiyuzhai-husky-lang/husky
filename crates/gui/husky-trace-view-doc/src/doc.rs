use crate::*;
use husky_trace_protocol::client_db::TraceDb;
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::{IsVisualProtocol, VisualActionBuffer};
use ui::IsUiComponent;

pub struct TraceViewDoc<VisualProtocol: IsVisualProtocol> {
    trace_db: TraceDb<VisualProtocol>,
    buffer_action: VisualActionBuffer<TraceViewAction>,
}

#[cfg(feature = "egui")]
impl<VisualProtocol: IsVisualProtocol, UiComponentConfig: HasTraceViewConfig>
    IsUiComponent<egui::Ui, UiComponentConfig> for TraceViewDoc<VisualProtocol>
{
    fn render(&mut self, ui: &mut egui::Ui, config: &UiComponentConfig) {
        // ui.label(text)
        ui.label("let");
        ui.label("let x = y");
        ui.label("let x = y");
        ui.label("let x = y");
        ui.end_row();
        ui.label("s");
        ui.end_row();
        ui.label("s");
        ui.end_row();
        ui.label("s");
        ui.end_row();
        ui.label("s");
        ui.end_row();
    }
}

pub trait HasTraceViewConfig {}

#[cfg(feature = "mock")]
pub type MockTraceViewDoc = TraceViewDoc<MockVisualProtocol>;

#[cfg(feature = "mock")]
impl TraceViewDoc<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            trace_db: TraceDb::new_mock(),
            buffer_action: Default::default(),
        }
    }
}
