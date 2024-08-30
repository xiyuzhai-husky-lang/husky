use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: FigureUi<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(in crate::view) fn render_figure(&mut self, ui: &mut egui::Ui) {
        let visual_synchrotron = self.trace_synchrotron.visual_synchrotron();
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::hover());
        ui.allocate_ui_at_rect(rect, |ui| {
            self.figure
                .ui(rect, visual_synchrotron, self.visual_ui_cache, ui)
        });
    }
}
