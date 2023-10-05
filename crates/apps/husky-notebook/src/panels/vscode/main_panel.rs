use super::*;

impl NotebookApp {
    pub(super) fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        self.docs.render(ui, &mut self.dock_state)
    }
}

struct TabViewer {}
