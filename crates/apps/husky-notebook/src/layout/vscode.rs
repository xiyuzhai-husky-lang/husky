use super::*;

impl NotebookApp {
    pub(super) fn render_vscode_panels(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar")
            .resizable(false)
            .frame(self.menu_bar_frame())
            .show(ctx, |ui| self.render_menu_bar(ctx, ui));
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| self.render_status_bar(ctx, ui));
        self.render_middle_ground(ctx);
    }

    fn render_middle_ground(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("activity_bar")
            .resizable(false)
            .frame(self.settings.activity_bar_frame())
            .show(ctx, |ui| self.render_activity_bar(ctx, ui));

        egui::SidePanel::left("activity_view")
            .resizable(false)
            .frame(self.explorer_frame())
            .show(ctx, |ui| self.render_activity_view(ctx, ui));
        egui::SidePanel::right("figure_view")
            .resizable(false)
            .show(ctx, |ui| self.render_assist_view(ctx, ui));
        egui::CentralPanel::default()
            .frame(self.settings.main_panel_frame())
            .show(ctx, |ui| self.render_docs_view(ui));
    }
}
