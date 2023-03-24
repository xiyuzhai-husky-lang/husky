mod vscode;

use crate::*;
use egui::{Color32, Vec2, Visuals};

impl HuskyNotebookApp {
    pub(crate) fn render_panels(&mut self, ctx: &egui::Context) {
        match self.config.layout().high_level() {
            HuskyNotebookHighLevelLayout::Vscode => self.render_vscode_panels(ctx),
        }
    }
}

// ui.heading("Husky Notebook");
// ui.horizontal(|ui| {
//     let name_label = ui.label("Your name: ");
//     // ui.text_edit_singleline(&mut self.name)
//     //     .labelled_by(name_label.id);
//     CodeEdit::singleline(&mut self.name).ui(ui);
// });
// ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
// if ui.button("Click each year").clicked() {
//     self.age += 1;
// }
// ui.label(format!("Hello '{}', age {}", self.name, self.age));
// ui.label(format!(
//     "Formula: {}xdx",
//     husky_unicode_symbols::opr::OPR_INTEGRAL
// ))
