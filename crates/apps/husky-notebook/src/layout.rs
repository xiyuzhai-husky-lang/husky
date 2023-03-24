mod vscode;

use crate::*;
use egui::{Color32, Vec2, Visuals};

impl HuskyNotebookApp {
    pub(crate) fn render_layout(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.spacing_mut().item_spacing = Vec2 { x: 0., y: 0. };
        ui.visuals_mut().extreme_bg_color = Color32::RED;
        ui.visuals_mut().code_bg_color = Color32::RED;
        match self.config.layout().high_level() {
            HuskyNotebookHighLevelLayout::Vscode => self.render_vscode_high_level_layout(ctx, ui),
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
