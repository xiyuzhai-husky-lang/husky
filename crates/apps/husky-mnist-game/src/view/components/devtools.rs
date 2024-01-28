use super::*;
use egui::Ui;

impl MnistApp {
    pub(in super::super) fn devtools_ui(&self, ui: &mut Ui) {
        ui.label(format!(
            r#"input_id = {:#?}
channels = {:#?}"#,
            self.input_id(),
            self.channels
        ));
    }
}
