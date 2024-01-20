use egui::Frame;

use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    <TraceProtocol::Figure as IsFigure>::View<'a>: egui::Widget,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_pedestal(&mut self, ui: &mut egui::Ui) {
        TopBottomPanel::bottom(ui.auto_id_with("pedestal"))
            .frame(Frame::none().inner_margin(2.0))
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "pedestal = {:?}",
                        self.trace_synchrotron.pedestal()
                    ))
                })
            });
    }
}
