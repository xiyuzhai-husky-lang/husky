use super::*;

impl NotebookApp {
    pub(crate) fn render_docs_view(&mut self, ui: &mut egui::Ui) {
        let style = egui_dock::Style::from_egui(ui.style().as_ref());
        egui_dock::DockArea::new(&mut self.dock_state)
            .style(style)
            .show_inside(
                ui,
                &mut DocsView {
                    docs: &mut self.docs,
                    action_buffer: &mut self.action_buffer,
                    settings: &mut self.settings,
                },
            )
    }
}

struct DocsView<'a> {
    docs: &'a mut Docs,
    action_buffer: &'a mut NotebookActionBuffer,
    settings: &'a mut NotebookSettings,
}

impl<'a> egui_dock::TabViewer for DocsView<'a> {
    type Tab = DocTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.docs
            .component_mut(tab.id())
            .render(ui, self.settings, self.action_buffer);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        self.docs.doc_arena()[tab.id()].title().into()
    }
}
