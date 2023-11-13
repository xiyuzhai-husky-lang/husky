mod vscode;

use crate::*;


impl NotebookApp {
    pub(crate) fn render_panels(&mut self, ctx: &egui::Context) {
        match self.settings.layout().high_level() {
            HuskyNotebookHighLevelLayout::Vscode => self.render_vscode_panels(ctx),
        }
    }
}
