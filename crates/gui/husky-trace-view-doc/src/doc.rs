use crate::*;
use husky_trace_protocol::client_db::TraceDb;
use husky_visual_protocol::{IsVisualProtocol, VisualActionBuffer};
use ui::{IsUi, IsUiComponent};

pub struct TraceViewDoc<VisualProtocol: IsVisualProtocol> {
    client_db: TraceDb<VisualProtocol>,
    buffer_action: VisualActionBuffer<TraceViewAction>,
}

impl<VisualProtocol: IsVisualProtocol, Ui: IsUi, UiComponentConfig: HasTraceViewConfig>
    IsUiComponent<Ui, UiComponentConfig> for TraceViewDoc<VisualProtocol>
{
    fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig) -> <Ui as IsUi>::Response {
        todo!()
    }
}

pub trait HasTraceViewConfig {}
