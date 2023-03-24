use egui::{Align, Layout, Vec2};

use super::*;

impl HuskyNotebookApp {
    pub(super) fn render_vscode_panels(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar")
            .frame(self.menu_bar_frame())
            .show(ctx, |ui| self.render_menu_bar(ctx, ui));
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| self.render_status_bar(ctx, ui));
        self.render_middle_ground(ctx);
    }

    fn render_middle_ground(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("activity_bar")
            .frame(self.activity_bar_frame())
            .show(ctx, |ui| self.render_activity_bar(ctx, ui));

        egui::SidePanel::left("activity_view")
            .frame(self.activity_view_frame())
            .show(ctx, |ui| self.render_activity_view_ui(ctx, ui));
        egui::SidePanel::right("figure_view").show(ctx, |ui| self.render_figure_view(ctx, ui));
        egui::CentralPanel::default().show(ctx, |ui| self.render_main_view(ctx, ui));
    }

    fn render_main_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("todo: main view");
    }

    fn render_figure_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label("todo: figure view");
    }
}
