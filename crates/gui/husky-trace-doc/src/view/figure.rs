use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_figure(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none().fill(Color32::WHITE).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "available size for figure = {:?}",
                        ui.available_size()
                    ))
                });
                ui.allocate_space(ui.available_size());
            })
        });
    }
}
