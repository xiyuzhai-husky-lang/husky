mod help;
mod standard;

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
    pub(crate) fn render_facade(mut self, ui: &mut egui::Ui) {
        match self.facade() {
            TraceDocFacade::Standard => self.render_standard_facade(ui),
            TraceDocFacade::Help => self.render_help_facade(ui),
        }
    }
}

#[derive(Debug)]
pub enum TraceDocFacade {
    Standard,
    Help,
}

impl Default for TraceDocFacade {
    fn default() -> Self {
        Self::Standard
    }
}
