use super::*;
use egui::CentralPanel;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: FigureUi<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_figure(&mut self, ui: &mut egui::Ui) {
        let visual_synchrotron = self.trace_synchrotron.visual_synchrotron();
        SidePanel::right(ui.auto_id_with("central_right"))
            .frame(Frame::none().inner_margin(0.0))
            .resizable(false)
            .exact_width(ui.available_width() * (1.0 - 1.0 / 1.618))
            .show_inside(ui, |ui| ui.label("todo: specific figure"));
        CentralPanel::default().show_inside(ui, |ui| {
            self.generic_figure
                .figure_ui(visual_synchrotron, self.visual_ui_cache, ui)
        });
    }
}
