use super::*;

impl NotebookApp {
    pub(crate) fn menu_bar_frame(&self) -> egui::Frame {
        egui::Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: egui::Color32::WHITE,
            stroke: Default::default(),
        }
    }

    pub(crate) fn render_menu_bar(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
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
