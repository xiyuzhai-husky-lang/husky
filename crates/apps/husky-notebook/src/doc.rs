mod action;
mod arena;
mod tab;

use self::action::*;
use self::arena::*;
pub(crate) use self::tab::*;
use super::*;
use egui_dock::DockState;
use husky_trace_view_doc::doc::{HasTraceViewConfig, MockTraceViewDoc, TraceViewDoc};
use ui::{IsUiComponent, UiComponent};

#[derive(Default)]
pub(crate) struct Docs {
    docs: DocArena,
    action_buffer: DocsActionBuffer,
}

impl Docs {
    pub(crate) fn render(&mut self, ui: &mut egui::Ui, dock_state: &mut DockState<DocTab>) {
        let style = egui_dock::Style::from_egui(ui.style().as_ref());
        egui_dock::DockArea::new(dock_state)
            .style(style)
            .show_inside(ui, self)
    }
}

impl Docs {
    pub fn new(dock_state: &mut DockState<DocTab>) -> Self {
        let mut this = Self {
            // ad hoc
            ..Default::default()
        };
        this.add(
            Doc {
                title: "mock trace view doc".to_string(),
                component: UiComponent::new(MockTraceViewDoc::new_mock()),
            },
            dock_state,
        );
        this.add(
            Doc {
                title: "hello_doc2".to_string(),
                component: UiComponent::new(AdHocUiComponent {}),
            },
            dock_state,
        );
        this.add(
            Doc {
                title: "hello_doc3".to_string(),
                component: UiComponent::new(AdHocUiComponent {}),
            },
            dock_state,
        );
        this
    }
}

pub struct AdHocUiComponent {}

impl<SuperActionBuffer> IsUiComponent<egui::Ui, AdHocUiComponentConfig, SuperActionBuffer>
    for AdHocUiComponent
{
    fn render(
        &mut self,
        ui: &mut egui::Ui,
        config: &AdHocUiComponentConfig,
        super_action_buffer: &mut SuperActionBuffer,
    ) {
        ui.label("Ui Component Context");
    }
}

impl Docs {
    pub(crate) fn add(&mut self, doc: Doc, dock_state: &mut DockState<DocTab>) {
        let id = self.docs.alloc(doc);
        dock_state.push_to_focused_leaf(DocTab::new(id))
    }
}

impl egui_dock::TabViewer for Docs {
    type Tab = DocTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        self.docs[tab.id()].component.render(
            ui,
            &AdHocUiComponentConfig {},
            &mut self.action_buffer,
        );
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        // (&*tab).into()
        format!("Title of doc{}", tab.id().index()).into()
    }
}

pub struct Doc {
    title: String,
    component: UiComponent<egui::Ui, AdHocUiComponentConfig, DocsActionBuffer>,
}

pub struct AdHocUiComponentConfig {}

impl HasTraceViewConfig for AdHocUiComponentConfig {}
