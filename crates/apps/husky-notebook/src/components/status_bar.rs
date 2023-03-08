use super::*;

impl HuskyNotebookApp {
    pub(crate) fn render_status_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("todo: status bar");
    }
}
