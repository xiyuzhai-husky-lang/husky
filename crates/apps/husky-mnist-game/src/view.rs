pub mod components;
pub mod layout;

use crate::*;

impl eframe::App for MnistApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.layout(ctx);
    }
}
