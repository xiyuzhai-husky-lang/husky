use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_pedestal(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none().inner_margin(1.0).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "pedestal = {:?}",
                    self.trace_synchrotron.pedestal()
                ));
                ui.allocate_space(ui.available_size())
            })
        });
    }
}
