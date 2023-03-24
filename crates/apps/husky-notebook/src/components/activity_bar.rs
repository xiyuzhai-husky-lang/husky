use super::*;

impl HuskyNotebookApp {
    pub(crate) fn activity_bar_frame(&self) -> egui::Frame {
        egui::Frame::none()
    }

    pub(crate) fn render_activity_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("todo: activity bar");
    }
}
