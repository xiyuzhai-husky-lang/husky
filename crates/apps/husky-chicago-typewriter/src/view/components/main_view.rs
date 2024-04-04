use crate::{action::TypeAction, app::ChicagoTypewriterApp};

struct MainView {}

impl ChicagoTypewriterApp {
    pub(crate) fn render_main_view(&mut self, ui: &mut egui::Ui) {
        ui.label("Theorem.");
        ui.label("x=x+1");
        ui.label("Proof.");
        ui.label("x=x=x+1");
    }
}
