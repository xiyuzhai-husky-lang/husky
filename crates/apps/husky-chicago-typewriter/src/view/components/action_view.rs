use crate::{action::TypeAction, app::ChicagoTypewriterApp};

struct ActionView<'a> {
    actions: &'a [TypeAction],
}

impl ChicagoTypewriterApp {
    pub(crate) fn render_action_view(&mut self, ui: &mut egui::Ui) {
        let (actions, selected) = self.suggested_actions();
        ui.label("action_view");
        ui.vertical(|ui| {
            for (i, action) in actions.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}.", i + 1));
                    ui.label(action.code());
                });
            }
        });
    }
}
