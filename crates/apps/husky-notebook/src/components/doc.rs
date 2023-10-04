mod docs;
mod tab;

use self::docs::*;
use self::tab::*;
use egui_dock::DockState;
use ui::UiComponent;

pub(crate) struct DocsDock {
    dock_state: DockState<DocTab>,
    docs: Docs,
}

impl DocsDock {
    pub(crate) fn render(&mut self, ui: &mut egui::Ui) {
        let style = egui_dock::Style::from_egui(ui.style().as_ref());
        egui_dock::DockArea::new(&mut self.dock_state)
            .style(style)
            .show_inside(ui, &mut self.docs)
    }
}

impl Default for DocsDock {
    fn default() -> Self {
        Self {
            // ad hoc
            dock_state: DockState::new(vec![]),
            docs: Docs::default(),
        }
    }
}

impl egui_dock::TabViewer for Docs {
    type Tab = DocTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self[tab.id()].component.render(ui);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        // (&*tab).into()
        format!("Title of doc{}", tab.id().index()).into()
    }
}

pub struct Doc {
    title: String,
    component: UiComponent<egui::Ui>,
}
