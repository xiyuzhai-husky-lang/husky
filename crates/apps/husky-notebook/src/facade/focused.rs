use super::*;

impl NotebookApp {
    pub(super) fn render_focused_facade(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar")
            .resizable(false)
            .frame(self.menu_bar_frame())
            .show(ctx, |ui| self.render_menu_bar(ctx, ui));
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| self.render_status_bar(ctx, ui));
        self.render_middle_facade(ctx);
    }

    fn render_middle_facade(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_facade")
            .resizable(false)
            .show(ctx, |ui| self.render_right_facade(ctx, ui));
        egui::CentralPanel::default()
            .frame(self.settings.main_panel_frame())
            .show(ctx, |ui| self.render_docs_view(ui));
    }

    fn render_right_facade(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::TopBottomPanel::bottom("log_view")
            .resizable(false)
            .frame(self.settings.log_view_frame())
            .show_inside(ui, |ui| self.render_log_view(ctx, ui));
        self.render_assist_view(ctx, ui)
    }
}
