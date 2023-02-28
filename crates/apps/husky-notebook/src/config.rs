mod background;
mod foreground;
mod layout;

pub(crate) use self::background::*;
pub(crate) use self::foreground::*;
pub(crate) use self::layout::*;

#[derive(Default, PartialEq, Eq)]
pub(crate) struct HuskyNotebookConfig {
    background: HuskyNotebookBackgroundConfig,
    foreground: HuskyNotebookForegroundConfig,
    layout: HuskyNotebookLayoutConfig,
}

impl HuskyNotebookConfig {
    pub(crate) fn layout(&self) -> &HuskyNotebookLayoutConfig {
        &self.layout
    }
}
