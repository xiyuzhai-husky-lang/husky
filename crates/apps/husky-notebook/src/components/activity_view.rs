use egui::{Color32, RichText};

use super::*;

impl HuskyNotebookApp {
    pub(crate) fn render_activity_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.visuals_mut().extreme_bg_color = Color32::RED;
        ui.visuals_mut().override_text_color = Some(Color32::RED);
        ui.label(RichText::new("Activity view").color(Color32::RED));
    }
}
