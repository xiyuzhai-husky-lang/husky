use super::*;
use eframe::egui::{CentralPanel, SidePanel};

impl ChicagoTypewriterApp {
    pub(crate) fn render_panels(&mut self, ctx: &egui::Context) {
        SidePanel::right("action view").show(ctx, |ui| self.render_action_view(ui));
        CentralPanel::default().show(ctx, |ui| self.render_main_view(ui));
    }
}
