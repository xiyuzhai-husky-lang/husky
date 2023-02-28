#[derive(Default, PartialEq, Eq)]
pub(crate) struct HuskyNotebookLayoutConfig {
    high_level: HuskyNotebookHighLevelLayout,
}

impl HuskyNotebookLayoutConfig {
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
