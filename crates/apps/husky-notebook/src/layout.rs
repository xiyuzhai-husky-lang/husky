mod vscode;

use crate::*;
use egui::{Color32, Vec2, Visuals};

impl NotebookApp {
    pub(crate) fn render_panels(&mut self, ctx: &egui::Context) {
        match self.settings.layout().high_level() {
            HuskyNotebookHighLevelLayout::Vscode => self.render_vscode_panels(ctx),
        }
    }
}
