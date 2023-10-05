use crate::*;
use husky_trace_protocol::client_db::TraceDb;
#[cfg(feature = "mock")]
use husky_visual_protocol::mock::MockVisualProtocol;
use husky_visual_protocol::{IsVisualProtocol, VisualActionBuffer};
use ui::{IsUi, IsUiComponent};

pub struct TraceViewDoc<VisualProtocol: IsVisualProtocol> {
    trace_db: TraceDb<VisualProtocol>,
    buffer_action: VisualActionBuffer<TraceViewAction>,
}

impl<VisualProtocol: IsVisualProtocol, Ui: IsUi, UiComponentConfig: HasTraceViewConfig>
    IsUiComponent<Ui, UiComponentConfig> for TraceViewDoc<VisualProtocol>
{
    fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig) -> <Ui as IsUi>::Response {
        ui.label("s")
    }
}

pub trait HasTraceViewConfig {}

#[cfg(feature = "mock")]
impl TraceViewDoc<MockVisualProtocol> {
    pub fn new_mock() -> Self {
        Self {
            trace_db: TraceDb::new_mock(),
            buffer_action: Default::default(),
        }
    }
}
