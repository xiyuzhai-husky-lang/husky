use self::components::traces::TracesView;
use super::*;
use egui::{vec2, Color32, Frame, Sense, SidePanel, TopBottomPanel, Vec2, Widget};
use husky_trace_protocol::figure::FigureUi;

impl MnistApp {
    pub(super) fn layout(&mut self, ctx: &egui::Context) {
        SidePanel::left("trace view").show(ctx, |ui| {
            let channel = self.current_channel_mut();
            TracesView::new(channel.trace_selection_mut()).ui(ui);
        });
        SidePanel::right("devtools").show(ctx, |ui| {
            ui.allocate_space(vec2(300.0, 0.0));
            self.devtools_ui(ui)
        });
        TopBottomPanel::bottom("control bar")
            .frame(Frame::none().inner_margin(5.0))
            .show(ctx, |ui| ui.vertical(|ui| self.control_mut().ui(ui)));
        self.layout_channels(ctx);
    }

    fn layout_channels(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let grid_size = ui.available_height().min((ui.available_width() * 0.7)) / 2.0;
            egui::Grid::new("channels").num_columns(2).show(ui, |ui| {
                let number_of_rows = 2;
                let number_of_columns = 2;
                for i in 0..number_of_rows {
                    for j in 0..number_of_columns {
                        let (_, response) =
                            ui.allocate_exact_size(Vec2::splat(grid_size), Sense::hover());
                        ui.allocate_ui_at_rect(response.rect, |ui| {
                            let pedestal = self.control.pedestal();
                            let op_time = self.control.op_time();
                            let channel = &self.channels[i * number_of_columns + j];
                            let set = channel.trace_selection().set();
                            // debug:
                            use crate::trace::Trace;
                            use husky_task_interface::val_repr::ValReprInterface;
                            use husky_trace_protocol::id::TraceId;
                            channel
                                .figure(pedestal, op_time, &self.db, &mut self.visual_synchrotron)
                                .figure_ui(
                                    &self.visual_synchrotron,
                                    self.ui_cache.figure_ui_cache_mut(),
                                    ui,
                                )
                        });
                    }
                    ui.end_row()
                }
            })
        });
    }
}
