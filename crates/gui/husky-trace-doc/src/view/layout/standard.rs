use egui::CentralPanel;
use husky_trace_protocol::caryatid::CaryatidUi;

use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,

    TraceProtocol::Figure: FigureUi<egui::Ui>,
    TraceProtocol::Caryatid: CaryatidUi<Ui>,
    Settings: HasTraceDocSettings,
{
    pub(crate) fn render_standard_facade(mut self, ui: &mut egui::Ui) {
        TopBottomPanel::bottom(ui.next_auto_id())
            .frame(Frame::none().inner_margin(Margin {
                left: 2.0,
                right: 2.0,
                top: 4.0, // ad hoc, investigate the implementation of egui to understand why this is needed
                bottom: 2.0,
            }))
            .show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| self.render_caryatid(ui))
            });
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
        CentralPanel::default().show_inside(ui, |ui| self.render_figure(ui));
    }
}
