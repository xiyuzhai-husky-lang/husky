use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(crate) fn render_standard_layout(mut self, ui: &mut egui::Ui) {
        TopBottomPanel::bottom(ui.next_auto_id()).show_inside(ui, |ui| self.render_pedestal(ui));
        self.render_central_region(ui);
    }

    fn render_central_region(&mut self, ui: &mut egui::Ui) {
        SidePanel::right(ui.auto_id_with("central_right"))
            .frame(Frame::none().inner_margin(0.0))
            .exact_width(ui.available_width() / 2.0)
            .resizable(false)
            .exact_width(ui.available_width() / 2.0)
            .show_inside(ui, |ui| self.render_central_right_region(ui));
        self.render_forest(ui);
    }

    fn render_central_right_region(&mut self, ui: &mut egui::Ui) {
        TopBottomPanel::bottom(ui.next_auto_id()).show_inside(ui, |ui| self.render_devtools(ui));
        self.render_figure(ui);
    }
}
