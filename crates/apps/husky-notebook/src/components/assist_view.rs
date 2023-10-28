use super::*;

impl NotebookApp {
    pub(crate) fn render_assist_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("todo: assist view");
        ui.allocate_space(ui.available_size());
    }
}
