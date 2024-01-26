use self::components::traces::TracesView;
use super::*;
use egui::{Color32, Frame, Sense, SidePanel, TopBottomPanel, Vec2, Widget};

impl MnistApp {
    pub(super) fn layout(&mut self, ctx: &egui::Context) {
        SidePanel::left("trace view").show(ctx, |ui| {
            let channel = self.current_channel_mut();
            TracesView::new(channel.trace_selection_mut()).ui(ui);
        });
        TopBottomPanel::bottom("control bar")
            .frame(Frame::none().inner_margin(5.0))
            .show(ctx, |ui| ui.vertical(|ui| self.control_mut().ui(ui)));
        self.layout_channels(ctx);
    }

    fn layout_channels(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let grid_size = ui.available_height().min(ui.available_width()) / 2.0;
            egui::Grid::new("channels").num_columns(2).show(ui, |ui| {
                let number_of_rows = 2;
                let number_of_columns = 2;
                for i in 0..number_of_rows {
                    for j in 0..number_of_columns {
                        let (_, response) =
                            ui.allocate_exact_size(Vec2::splat(grid_size), Sense::hover());
                        ui.allocate_ui_at_rect(response.rect, |ui| {
                            self.channels[i * number_of_columns + j]
                                .figure_view(&self.db, &mut self.ui_cache)
                                .ui(ui)
                        });
                    }
                    ui.end_row()
                }
            })
        });
    }
}
