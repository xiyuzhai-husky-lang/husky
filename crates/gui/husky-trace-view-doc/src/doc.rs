use crate::*;
use husky_task::{visual::ActionBuffer, IsTask};
use husky_trace_protocol::client_db::TraceDb;
use ui::{IsUi, IsUiComponent};

pub struct TraceViewDoc<Task: IsTask> {
    client_db: TraceDb<Task>,
    buffer_action: ActionBuffer<TraceViewAction>,
}

impl<Task: IsTask, Ui: IsUi, UiComponentConfig: HasTraceViewConfig>
    IsUiComponent<Ui, UiComponentConfig> for TraceViewDoc<Task>
{
    fn render(&mut self, ui: &mut Ui, config: &UiComponentConfig) -> <Ui as IsUi>::Response {
        todo!()
    }
}

pub trait HasTraceViewConfig {}
