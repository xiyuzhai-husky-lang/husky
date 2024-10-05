use super::*;
use ::egui::*;
use husky_trace_protocol::{caryatid::CaryatidUi, figure::FigureUi, protocol::IsTraceProtocol};
use view::TraceDocView;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: FigureUi<Ui>,
    TraceProtocol::Caryatid: CaryatidUi<Ui>,
    Settings: HasTraceDocSettings,
{
    pub(super) fn render_help_facade(mut self, ui: &mut egui::Ui) {
        ui.label("help\n Alt+F fill cascade.");
    }
}
