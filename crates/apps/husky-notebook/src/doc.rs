mod arena;
mod tab;

pub(crate) use self::tab::*;

use self::arena::*;
use super::*;

use husky_gui::helpers::repaint_signal::EguiRepaintSignal;
use husky_trace_doc::doc::{MockTraceDoc};
use ui::{UiComponent};

pub struct Doc {
    title: String,
    component: UiComponent<egui::Ui, NotebookSettings, NotebookActionBuffer>,
}

impl Doc {
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }
}

#[derive(Default)]
pub(crate) struct Docs {
    doc_arena: DocArena,
}

impl Docs {
    pub(crate) fn doc_arena(&self) -> &DocArena {
        &self.doc_arena
    }

    pub(crate) fn component_mut(
        &mut self,
        id: DocId,
    ) -> &mut UiComponent<egui::Ui, NotebookSettings, NotebookActionBuffer> {
        &mut self.doc_arena[id].component
    }
}

impl NotebookApp {
    pub fn add_default_docs(&mut self, ctx: &egui::Context) {
        self.add_doc(Doc {
            title: "mock trace view doc".to_string(),
            component: UiComponent::new(MockTraceDoc::new_mock(
                self.tokio_runtime.clone(),
                EguiRepaintSignal::new(ctx.clone()),
            )),
        });
        // self.add_doc(Doc {
        //     title: "Settings".to_string(),
        //     component: UiComponent::new(NotebookSettingsView),
        // })
    }

    pub(crate) fn add_doc(&mut self, doc: Doc) {
        let id = self.docs.doc_arena.alloc(doc);
        self.dock_state.push_to_focused_leaf(DocTab::new(id))
    }
}
