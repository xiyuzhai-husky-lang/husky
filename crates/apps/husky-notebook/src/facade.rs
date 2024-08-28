mod focused;

use crate::*;

impl NotebookApp {
    pub(crate) fn render_facade(&mut self, ctx: &egui::Context) {
        match self.settings.layout().high_level() {
            NotebookFacadeLayout::Focused => self.render_focused_facade(ctx),
        }
    }
}
