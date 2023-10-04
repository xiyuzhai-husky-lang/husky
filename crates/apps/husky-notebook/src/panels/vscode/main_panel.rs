use super::*;

impl HuskyNotebookApp {
    pub(super) fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        self.docs_dock.render(ui)
    }
}

struct TabViewer {}
