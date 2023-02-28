mod vscode;

use crate::*;

impl HuskyNotebookApp {
    pub(crate) fn render_layout(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
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
