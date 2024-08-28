use super::*;

impl NotebookApp {
    pub(super) fn render_focused_facade(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(self.settings.main_panel_frame())
            .show(ctx, |ui| self.render_docs_view(ui));
    }
}
