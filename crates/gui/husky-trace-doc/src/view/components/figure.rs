use super::*;
use egui::{load::Bytes, pos2, Frame, Image, ImageSource, Rect};
use husky_trace_protocol::figure::IsFigure;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: FigureUi<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_figure(&mut self, ui: &mut egui::Ui) {
        let visual_synchrotron = self.trace_synchrotron.visual_synchrotron();
        self.figure
            .figure_ui(visual_synchrotron, self.figure_ui_cache, ui)
    }
}
