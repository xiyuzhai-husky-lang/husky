use egui::{Color32, RichText};

use super::*;

impl NotebookApp {
    pub(crate) fn explorer_frame(&self) -> egui::Frame {
        egui::Frame::none()
    }

    #[inline(always)]
    pub(crate) fn render_explorer_ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.label(RichText::new("Explorer"));
    }
}
