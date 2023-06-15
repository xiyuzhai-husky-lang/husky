use crate::*;

impl TemplateMatchingViewerApp {
    pub(super) fn render_menu_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar_panel")
            .frame(self.menu_bar_frame())
            .show(ctx, |ui| self.render_menu_bar_content(ctx, ui));
    }

    fn menu_bar_frame(&self) -> egui::Frame {
        egui::Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: egui::Color32::WHITE,
            stroke: Default::default(),
        }
    }

    fn render_menu_bar_content(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("File");
            ui.label("Edit");
            ui.label("Selection");
            ui.label("View");
            ui.label("Go");
            ui.label("Run");
            ui.label("Terminal");
            ui.label("Help");
        });
    }
}
