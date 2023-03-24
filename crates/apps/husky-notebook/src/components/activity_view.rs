use egui::{Color32, RichText};

use super::*;

impl HuskyNotebookApp {
    pub(crate) fn activity_view_frame(&self) -> egui::Frame {
        egui::Frame::none()
    }

    #[inline(always)]
    pub(crate) fn render_activity_view_ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label(RichText::new("Activity view").color(Color32::RED));
    }
}
