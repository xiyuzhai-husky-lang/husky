use crate::app::ChicagoTypewriterApp;
use egui::FontFamily::Proportional;
use egui::{TextStyle::*, *};

struct MainView {}

impl ChicagoTypewriterApp {
    pub(crate) fn render_main_view(&mut self, ui: &mut egui::Ui) {
        ui.style_mut().text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ui.style_mut()
            .visuals
            .widgets
            .noninteractive
            .fg_stroke
            .color = Color32::WHITE;
        ui.label("Theorem.");
        ui.label("x=x+1");
        ui.label("Proof.");
        ui.label("x=x=x+1");
    }
}
