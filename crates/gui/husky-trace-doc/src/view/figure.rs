use egui::Frame;

use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_figure(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none().inner_margin(1.0).show(ui, |ui| {
            egui::Frame::none()
                .fill(Color32::WHITE)
                .stroke((1.0, Color32::BLACK))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.allocate_ui(
                            Vec2::new(ui.available_width(), ui.available_height() * 0.7),
                            |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "available size for canvas = {:?}",
                                        ui.available_size()
                                    ));
                                    ui.label("canvas");
                                });
                                ui.allocate_space(ui.available_size())
                            },
                        );
                        egui::Frame::none()
                            .fill(Color32::WHITE)
                            .stroke((1.0, Color32::BLACK))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let available_size = ui.available_size();
                                    ui.label(format!(
                                        "available size for control = {available_size:?}"
                                    ))
                                });
                                ui.allocate_space(ui.available_size())
                            });
                        ui.allocate_space(ui.available_size());
                    })
                })
        });
    }
}
