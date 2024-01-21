use super::*;
use egui::{load::Bytes, pos2, Frame, Image, ImageSource, Rect};
use husky_trace_protocol::figure::IsFigure;
use ui::visual_widget::VisualWidget;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_figure(&mut self, ui: &mut egui::Ui) {
        if let Some(figure) = self.figure {
            let visual_synchrotron = self.trace_synchrotron.visual_synchrotron();
            figure.ui(visual_synchrotron, self.ui_cache, ui)
        }
    }
}
