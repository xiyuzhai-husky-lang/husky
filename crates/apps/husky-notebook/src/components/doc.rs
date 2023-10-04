mod docs;
mod tab;

use self::docs::*;
use self::tab::*;
use egui_dock::DockState;
use ui::{IsUiComponent, UiComponent};

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
        let mut this = Self {
            // ad hoc
            dock_state: DockState::new(vec![]),
            docs: Docs::default(),
        };
        this.add(Doc {
            title: "hello_doc1".to_string(),
            component: UiComponent::new(AdHocUiComponent {}),
        });
        this.add(Doc {
            title: "hello_doc2".to_string(),
            component: UiComponent::new(AdHocUiComponent {}),
        });
        this.add(Doc {
            title: "hello_doc3".to_string(),
            component: UiComponent::new(AdHocUiComponent {}),
        });
        this
    }
}

pub struct AdHocUiComponent {}

impl IsUiComponent<egui::Ui, AdHocUiComponentConfig> for AdHocUiComponent {
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        config: &AdHocUiComponentConfig,
    ) -> <egui::Ui as ui::IsUi>::Response {
        ui.label("Ui Component Context")
    }
}

impl DocsDock {
    pub(crate) fn add(&mut self, doc: Doc) {
        let id = self.docs.alloc(doc);
        self.dock_state.push_to_focused_leaf(DocTab::new(id))
    }
}

impl egui_dock::TabViewer for Docs {
    type Tab = DocTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self[tab.id()]
            .component
            .render(ui, &AdHocUiComponentConfig {});
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        // (&*tab).into()
        format!("Title of doc{}", tab.id().index()).into()
    }
}

pub struct Doc {
    title: String,
    component: UiComponent<egui::Ui, AdHocUiComponentConfig>,
}

pub struct AdHocUiComponentConfig {}
