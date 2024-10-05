use crate::hotkey::NotebookHotkeyAction;

pub enum NotebookAction {
    ToggleHelpFacade,
    ToggleDocHelpFacade,
}

impl NotebookAction {
    pub(crate) fn from_hotkey_action(
        argument: Option<usize>,
        action: NotebookHotkeyAction,
    ) -> Self {
        match action {
            NotebookHotkeyAction::ToggleHelpFacade => NotebookAction::ToggleHelpFacade,
            NotebookHotkeyAction::ToggleDocHelpFacade => NotebookAction::ToggleDocHelpFacade,
        }
    }
}

#[derive(Default)]
pub struct NotebookActionBuffer {
    actions: Vec<NotebookAction>,
}

impl NotebookActionBuffer {
    pub(crate) fn take_all(&mut self) -> Vec<NotebookAction> {
        std::mem::take(&mut self.actions)
    }
}
