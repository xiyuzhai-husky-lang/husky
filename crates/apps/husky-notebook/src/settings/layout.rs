use super::*;

#[derive(Default, PartialEq, Eq)]
pub(crate) struct HuskyNotebookLayoutSettings {
    high_level: HuskyNotebookHighLevelLayout,
}

impl HuskyNotebookLayoutSettings {
    pub fn high_level(&self) -> HuskyNotebookHighLevelLayout {
        self.high_level
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HuskyNotebookHighLevelLayout {
    Vscode,
}

impl Default for HuskyNotebookHighLevelLayout {
    fn default() -> Self {
        HuskyNotebookHighLevelLayout::Vscode
    }
}

impl NotebookSettings {
    pub(crate) fn layout(&self) -> &HuskyNotebookLayoutSettings {
        &self.layout
    }
}
