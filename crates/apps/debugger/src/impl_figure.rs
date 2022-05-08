mod figure_focus;
mod impl_figure_control;

pub use figure_focus::*;
pub use impl_figure_control::*;

use serde::Serialize;

use crate::*;

impl Debugger {
    pub async fn figure(&self, id: TraceId, focus: &Focus) -> FigureProps {
        self.runtime.lock().unwrap().figure(id, focus)
    }
}
