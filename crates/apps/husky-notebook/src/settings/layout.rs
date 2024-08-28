use super::*;

#[derive(Default, PartialEq, Eq)]
pub(crate) struct HuskyNotebookLayoutSettings {
    high_level: NotebookFacadeLayout,
}

impl HuskyNotebookLayoutSettings {
    pub fn high_level(&self) -> NotebookFacadeLayout {
        self.high_level
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NotebookFacadeLayout {
    Focused,
}

impl Default for NotebookFacadeLayout {
    fn default() -> Self {
        NotebookFacadeLayout::Focused
    }
}

impl NotebookSettings {
    pub(crate) fn layout(&self) -> &HuskyNotebookLayoutSettings {
        &self.layout
    }
}
